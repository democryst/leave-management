# --- Build Stage ---
FROM rust:1.80-slim as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release -p staff-service

# --- Final Stage ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/staff-service /app/staff-service

EXPOSE 8081

CMD ["/app/staff-service"]
