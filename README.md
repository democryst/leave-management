# 🕊️ Leave Management System v1.0

A production-grade, microservices-based Leave Management System built with **Rust**, **Next.js 15**, and **OpenTelemetry**. Designed for high performance, reliability, and observability in self-hosted environments.

## 🏗️ Architecture

The system follows **Hexagonal Architecture** (Ports & Adapters) to ensure domain logic remains isolated and testable.

```mermaid
graph TD
    User((User))
    Web[Next.js Dashboard]
    GW[API Gateway]
    Staff[Staff Service]
    Leave[Leave Service]
    Policy[Policy Service]
    DB[(PostgreSQL)]
    Jaeger[Jaeger Tracing]

    User --> Web
    Web --> GW
    GW -- Auth/IST --> Staff
    GW -- Auth/IST --> Leave
    Leave --> Policy
    Staff --> DB
    Leave --> DB
    Policy --> DB
    
    Web -.-> Jaeger
    GW -.-> Jaeger
    Staff -.-> Jaeger
    Leave -.-> Jaeger
    Policy -.-> Jaeger
```

## 🛠️ Tech Stack

### Frontend
- **Framework:** Next.js 15 (App Router)
- **Runtime:** Bun 1.3
- **Styling:** Vanilla CSS (Glassmorphism / Premium UI)
- **State Management:** TanStack Query v5
- **Observability:** OpenTelemetry v2.x (Browser SDK)

### Backend (Rust)
- **Framework:** Axum 0.7
- **Database:** SQLx 0.8 (PostgreSQL)
- **Precision:** BigDecimal
- **Security:** RS256 Internal Service Tokens (IST)
- **Tracing:** Tracing-Opentelemetry + OTLP

### Infrastructure
- **Containerization:** Docker & Docker Compose
- **Tracing Sink:** Jaeger (OTLP)
- **Deployment:** Self-Hosted / On-Premise

## ✨ Key Features

- **Microservices Orchestration:** Clean separation of Staff, Leave, and Policy domains.
- **Atomic Leave Requests:** Transactional integrity with balance checks using `BigDecimal`.
- **Full-Stack Tracing:** Visualize the entire request lifecycle from browser to database.
- **Service Mesh Security:** All internal calls secured via RS256-signed identity tokens.
- **Premium Dashboard:** Responsive 3-column grid with real-time status updates.

## 🚀 Getting Started

### Prerequisites
- [Bun](https://bun.sh/)
- [Rust](https://www.rust-lang.org/)
- [Docker & Docker Compose](https://www.docker.com/)

### 1. Initialize Infrastructure
```bash
docker-compose up -d
```

### 2. Configure Environment
Create a `.env` file in the root:
```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/leave_management
EDGE_JWT_SECRET=your_edge_secret
IST_PRIVATE_KEY="your_rs256_private_key"
```

### 3. Run Services
**Backend:**
```bash
cargo run --package gateway
cargo run --package staff-service
cargo run --package leave-service
cargo run --package policy-service
```

**Frontend:**
```bash
cd web
bun install
bun run dev
```

## 🔭 Observability
Access the **Jaeger UI** at `http://localhost:16686` to trace leave requests in real-time.

## 📜 License
Internal Organization Use Only.
