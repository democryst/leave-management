use std::sync::Arc;
use std::time::Duration;
use axum::{
    http::{StatusCode, Request},
    middleware::Next,
    response::IntoResponse,
};
use governor::{Quota, RateLimiter, state::DirectStateStore, state::NotKeyed};
use failsafe::{Config, Backoff, failure_policy::ConsecutiveFailures};

pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, DirectStateStore, governor::clock::DefaultClock>>;
pub type CircuitBreaker = failsafe::GenericStateMachine<Backoff, ConsecutiveFailures>;

pub struct ResilienceProvider {
    pub rate_limiter: SharedRateLimiter,
    pub staff_breaker: CircuitBreaker,
    pub leave_breaker: CircuitBreaker,
    pub policy_breaker: CircuitBreaker,
}

impl ResilienceProvider {
    pub fn new() -> Self {
        // Rate limit: 100 requests per second
        let quota = Quota::per_second(std::num::NonZeroU32::new(100).unwrap());
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        // Circuit Breaker: 5 consecutive failures opens for 30s
        let breaker_config = Config::new()
            .failure_policy(ConsecutiveFailures::new(5))
            .backoff(Backoff::constant(Duration::from_secs(30)));

        Self {
            rate_limiter,
            staff_breaker: breaker_config.clone().build(),
            leave_breaker: breaker_config.clone().build(),
            policy_breaker: breaker_config.build(),
        }
    }
}

pub async fn rate_limit_middleware<B>(
    axum::extract::State(limiter): axum::extract::State<SharedRateLimiter>,
    request: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    if limiter.check().is_ok() {
        next.run(request).await
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response()
    }
}
