# Security Architecture (ADR-001)

## 🛡️ Internal Service Tokens (IST)
The system employs a "Gateway-Issued Token" pattern to secure inter-service communication.

### 1. Token Exchange Flow
1. **Frontend** sends an **Edge JWT** (HS256/RS256 from Auth0/Identity Provider) to the Gateway.
2. **Gateway** validates the Edge JWT.
3. **Gateway** generates a short-lived **IST** signed with a private **RS256** key.
4. **Gateway** injects the IST into the `Authorization` header.
5. **Microservices** validate the IST using the Gateway's public key.

### 2. IST Payload
The IST contains the following claims:
- `sub`: User's unique ID.
- `role`: User's role (admin, manager, staff).
- `iss`: `leave-management-gateway`.
- `iat`: Issued at timestamp.
- `exp`: Expiration timestamp (typically 5-15 minutes).

### 3. Key Management
- **Private Key:** Stored in the Gateway environment (`IST_PRIVATE_KEY`).
- **Public Key:** Shared with all microservices via environment variables or a discovery endpoint.

## 🔒 PII Masking
Per our security guidelines, all PII (Personally Identifiable Information) such as emails and full names must be masked before being sent to the observability pipeline (traces/logs).

## 🔏 Database Security
- All sensitive data at rest is encrypted via PostgreSQL transparent data encryption (if supported by the host) or application-level encryption for specific fields.
- Connections use **TLS (rustls)** via SQLx features.
