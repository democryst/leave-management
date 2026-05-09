# Risk Register & Mitigation

| Risk | Impact | Probability | Mitigation Strategy |
|------|--------|-------------|---------------------|
| **Distributed Transactions:** Balance consistency across services. | High | Medium | Use atomic Postgres transactions within Leave service; Staff data is read-only for Leave. |
| **Trace Leakage:** PII appearing in OTel spans. | Medium | High | **gemma-sec** audit of OTel attributes; mandatory masking traits. |
| **Circuit Cascading:** One service failure bringing down Web. | High | Medium | Implement circuit breakers at the Gateway and inter-service clients. |
| **Auth Complexity:** Multi-tier JWT validation failure. | High | Low | Centralized Auth logic in Gateway; standard `jose` library across all Rust services. |
| **Data Silos:** Staff info needed by Leave service. | Medium | Medium | Leave service caches minimum required Staff IDs; Gateway handles join operations if needed (BFF pattern). |
