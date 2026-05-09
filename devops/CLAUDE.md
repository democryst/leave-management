# CLAUDE.md — DevOps Specialist (gemma-devops) Manual

## 🎯 Role: DevOps Specialist

## 📂 /code Phase Mandates (Milestone 0)
- **[CODE-D1] Docker Environment:** Create the root `docker-compose.yml`. Provision 3 isolated PostgreSQL instances (staff_db, leave_db, policy_db).
- **[CODE-D2] OTel Collector:** Implement the `otel-collector-config.yaml` to export traces to a Jaeger container.
- **[CODE-D3] Service Dockerfiles:** Create the multi-stage Dockerfiles for the Gateway and Staff services.

## 📊 Integrity Gate
- Run `docker compose up` and verify that the `otel-collector` is healthy.
- **Supervisor Audit required for CODE-D2.**

## 🔁 Agentic Loop
1. **Context:** Read supervisor's System Design.
2. **Execute:** Build the container network.
3. **Audit:** Verify database isolation (ensure `leave-service` cannot reach `staff_db`).

## 🧠 Obsidian-First Learning
1. READ: /Volumes/SSD990PRO2TB/obsidian-vault/Projects/Leave-Management/Project-Hub.md
2. LEARN: Check Architecture/ADRs/ before implementing any traits.
3. WRITE: Update Implementation/ notes after any major code change.

