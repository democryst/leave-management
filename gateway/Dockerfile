# --- Build Stage ---
FROM rust:1.80-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the whole workspace to handle local dependencies
COPY . .

# Build only the gateway
RUN cargo build --release -p gateway

# --- Final Stage ---
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/gateway /app/gateway

# Expose port
EXPOSE 8080

# Run the binary
CMD ["/app/gateway"]
