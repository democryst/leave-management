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
        // Configurable Rate limit
        let rate_limit = std::env::var("GATEWAY_RATE_LIMIT_PER_SEC")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        
        let quota = Quota::per_second(std::num::NonZeroU32::new(rate_limit).unwrap());
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        // Configurable Circuit Breaker
        let cb_threshold = std::env::var("GATEWAY_CB_THRESHOLD")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);
        
        let cb_timeout = std::env::var("GATEWAY_CB_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        let breaker_config = Config::new()
            .failure_policy(ConsecutiveFailures::new(cb_threshold))
            .backoff(Backoff::constant(Duration::from_secs(cb_timeout)));

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
