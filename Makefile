.PHONY: build test run-local docker-up docker-down k8s-deploy clean lint

# --- Variables ---
DC = docker-compose
K = kubectl

# --- Development ---
build:
	cargo build --workspace

test:
	cargo test --workspace

lint:
	cargo clippy --workspace -- -D warnings
	cargo fmt --all -- --check

run-local:
	@echo "Starting all services locally (direct execution)..."
	# Note: Requires env vars to be set manually or via a .env file
	cargo run -p gateway &
	cargo run -p staff &
	cargo run -p leave &
	cargo run -p policy &

# --- Docker ---
docker-up:
	$(DC) up --build -d

docker-down:
	$(DC) down

# --- Kubernetes ---
k8s-deploy:
	$(K) apply -f k8s/configmap.yaml
	$(K) apply -f k8s/secrets.yaml
	$(K) apply -f k8s/infrastructure.yaml
	$(K) apply -f k8s/services-deployment.yaml
	$(K) apply -f k8s/gateway-deployment.yaml
	# $(K) apply -f k8s/web-deployment.yaml # Add when web is finalized

# --- Cleanup ---
clean:
	cargo clean
	$(DC) down -v
