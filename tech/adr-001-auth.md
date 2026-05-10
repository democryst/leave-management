# ADR-001: Internal JWT Trust Chain

## Context
In a microservices environment, we need a way to propagate user identity and permissions from the Gateway to downstream services securely.

## Decision
We will implement a two-tier authentication system:
1. **Edge Auth:** Gateway validates user credentials (or Edge JWT from external provider) and issues/validates the token.
2. **Internal Auth:** For every downstream request, the Gateway generates a short-lived (15 min) Internal Service Token (IST) using an RS256 private key.
3. **Identity Header Injection:** As a performance optimization, the Gateway will also inject `X-User-Id` and `X-User-Role` headers into downstream requests after successful validation.
4. **Validation:** Microservices should validate the IST signature using the Gateway's Public Key for high-security operations, but can rely on `X-User-Id` for standard domain lookups within the trusted internal network.

## Consequences
- **Positive:** Services don't need to query the Staff service for every request to verify identity.
- **Positive:** Internal services can quickly access the `sub` and `role` without parsing the JWT body repeatedly.
- **Positive:** Strict actor-based AuthZ can be performed locally in the `services` layer.
- **Negative:** Headers can be spoofed if the internal network is compromised (mitigated by mandatory IST validation for write operations).

## Standards
- **Algorithm:** RS256.
- **Required Claims:** `sub` (Staff UUID), `role` (Staff Role), `iat`, `exp`.
- **Identity Headers:** `X-User-Id`, `X-User-Role`.
