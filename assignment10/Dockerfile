# -------------------------------
# Stage 1 — Build Application
# -------------------------------

FROM rust:1.89 as builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy src for dependency caching
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy source
RUN rm -rf src

# Copy actual source code
COPY . .

# Build production application
RUN cargo build --release

# -------------------------------
# Stage 2 — Runtime Image
# -------------------------------

FROM debian:bookworm-slim

WORKDIR /app

# Install required runtime libraries
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/target/release/assignment10 /app/assignment10

# Expose API port
EXPOSE 3000

# Run application
CMD ["./assignment10"]
