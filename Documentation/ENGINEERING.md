# Software Engineering Principles & Philosophy

This document outlines the core engineering principles followed in the development of the Leave Management System. It serves as a guide for maintainers and a foundation for the system's architecture.

---

## 1. The Core Pillars of Knowledge

### Computer Science Fundamentals
The "science" behind the code ensuring efficiency and scalability:
- **Data Structures:** Organized data handling (Arrays, Linked Lists, Trees, Hash Maps).
- **Algorithms:** Logic for problem solving (Sorting, Searching, Big O Notation).
- **Functional Theory:** [Category Theory & λ-calculus](./theory/FUNCTIONAL.md).
- **Formal Verification:** [Hoare Logic & Model Checking](./theory/VERIFICATION.md).
- **Operating Systems:** Low-level memory management, processes, and threading.

### System Design & Architecture
The "engineering" part—planning component communication:
- **Monolithic vs. Microservices:** Deciding on modularity (e.g., our Staff, Leave, and Policy services).
- **Databases:** Strategic choice of SQL (relational) vs. NoSQL based on data needs.
- **APIs:** Designing the "contracts" (REST/gRPC) that allow disparate services to communicate.

### Software Development Life Cycle (SDLC)
A professional process to ensure quality:
- **Requirements:** Defining user needs.
- **Design:** Technical blueprinting (ADRs).
- **Implementation:** The coding phase.
- **Testing:** Unit and integration tests to catch bugs early.
- **Security:** [The CIA Triad & Core Principles](./Security-Architecture.md) integrated into the SDLC.
- **Deployment & Maintenance:** Continuous operation and monitoring.

---

## 2. The Modern Tech Stack (LMS Implementation)

| Layer | Technologies | Purpose |
|-------|--------------|---------|
| **Frontend** | React (Next.js), Vanilla CSS | User interaction and visual experience. |
| **Backend** | Rust (Axum) | The logic, server, and "brain" of the app. |
| **Database** | PostgreSQL | Permanent, relational data storage. |
| **DevOps** | Docker, OTel, Jaeger | Packaging, hosting, and observability. |

---

## 3. The "Soft" Skills & Engineering Pro-Tips

### Problem Solving
Breaking massive, complex problems into tiny, manageable tasks (as seen in our 9-Stage Execution Plan).

### Code Reviews
Continuous feedback to ensure quality and maintainability.

### Communication
Explaining technical debt and realistic timelines to stakeholders.

> [!TIP]
> **Engineer's Pro-Tip:** The most expensive code is the code that is hard to read. Always write code for the human who has to maintain it six months from now—that human will likely be you!

---

## 4. Language References
To support polyglot expansion and maintain architectural standards across different runtimes:
- [Rust (Primary)](./languages/RUST.md)
- [Go (Golang)](./languages/GO.md)

---
*Derived from Sovereign Engineering Standards.*
