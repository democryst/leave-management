# CLAUDE.md — Backend Specialist (gemma-dev) Manual

## 🎯 Role: Domain Developer (Staff Service)

## 📂 /code Phase Mandates (Milestone 0)
- **[CODE-S1] Repository Implementation:** Build the `SqlxStaffRepository` in `internal/adapters/repository/`. Use `sqlx::query_as!` for compile-time checked queries.
- **[CODE-S2] Service Implementation:** Build the `StaffService` in `internal/core/services/`. Implement `onboard_staff` and `get_org_chart`.
- **[CODE-S3] Handler Implementation:** Build the Axum handlers in `internal/adapters/handler/`. Ensure all inputs pass through `validator`.
- **[CODE-S4] Migration:** Create the initial PostgreSQL migration in `migrations/` for the `staff` table.

## 📊 Integrity Gate
- Before submitting, run `cargo test` and ensure the `Mask` trait is applied to all Staff responses.
- **Supervisor Audit required for CODE-S1 before proceeding to CODE-S2.**

## 🔁 Agentic Loop
1. **Context:** Read supervisor's `task.md` and `spec/domain_model.md`.
2. **Execute:** Implement the tasks sequentially.
3. **Audit:** Verify your SQL queries against the schema design.

## 🧠 Obsidian-First Learning
1. READ: /Volumes/SSD990PRO2TB/obsidian-vault/Projects/Leave-Management/Project-Hub.md
2. LEARN: Check Architecture/ADRs/ before implementing any traits.
3. WRITE: Update Implementation/ notes after any major code change.

