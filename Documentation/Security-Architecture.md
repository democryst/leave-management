# Security Architecture & Principles

## 🏛️ Security Foundation: The CIA Triad
Every security measure in the Leave Management System is designed to protect one or more of these pillars:
- **Confidentiality:** Ensuring data is accessible only to authorized users (e.g., RS256 IST, Argon2 hashing).
- **Integrity:** Ensuring data is accurate and untampered (e.g., Digital Signatures, SQL Parameterization).
- **Availability:** Ensuring systems are resilient (e.g., Microservice redundancy, DDoS mitigation).

## 🛡️ Core Engineering Principles
- **Defense in Depth:** Multiple layers of protection (Gateway Auth -> Service Auth -> DB Access Control).
- **Least Privilege:** Users/Services only have the minimum access required (e.g., RBAC gating for Admin actions).
- **Zero Trust:** Every internal request must be authenticated via IST, regardless of source.
- **Delegatee-Aware Authorization:** In scenarios where a user acts "on behalf of" another (e.g., Approver Delegation), the target service must verify the relationship through a centralized identity check. The system uses a dedicated `system-token` to query the Delegation Registry, preventing permission circularity.

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

## ☣️ OWASP Top 10 Mitigations
| Vulnerability | Mitigation in LMS |
|---------------|-------------------|
| **Injection** | Using `sqlx` parameterized queries exclusively. |
| **Broken Access Control** | Centralized `admin_only_middleware` and role-based IST validation. |
| **Cryptographic Failures** | Enforcement of **Argon2id** for passwords and **RS256** for service tokens. |
| **Insecure Design** | Explicit ADR-led design process with formal verification in mind. |

## 🔑 Cryptography Standards
- **Symmetric:** Used for internal data encryption (AES-256-GCM).
- **Asymmetric:** **RS256** (RSA Signature with SHA-256) for Gateway-to-Service identity propagation.
- **Hashing:** **Argon2id** for secure password storage, preventing brute-force and GPU acceleration attacks.

---
*Verified against Sovereign Security Standards.*
