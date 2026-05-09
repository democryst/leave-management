# ADR-001: Internal JWT Trust Chain

## Context
In a microservices environment, we need a way to propagate user identity and permissions from the Gateway to downstream services securely.

## Decision
We will implement a two-tier authentication system:
1. **Edge Auth:** Gateway validates user credentials (Argon2) and issues an external JWT to the Web client.
2. **Internal Auth:** For every downstream request, the Gateway generates a short-lived (5 min) Internal Service Token (IST) using an RS256 private key.
3. **Validation:** Every microservice will possess the Public Key to validate the IST.

## Consequences
- **Positive:** Services don't need to query the Staff service for every request to verify identity.
- **Positive:** Strict actor-based AuthZ can be performed locally in the `services` layer.
- **Negative:** Key rotation becomes a cross-service management task.

## Standards
- **Algorithm:** RS256.
- **Required Claims:** `sub` (Staff UUID), `role` (Staff Role), `iat`, `exp`.
