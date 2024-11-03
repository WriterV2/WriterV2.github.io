# Planner stage - Compute dependencies and create recipe
FROM lukemathwalker/cargo-chef:latest-rust-1.75 AS planner
WORKDIR /app
# Copy the minimal files needed for dependency calculation
COPY Cargo.* ./
COPY src/ src/
RUN cargo chef prepare --recipe-path recipe.json

# Cache dependencies stage
FROM lukemathwalker/cargo-chef:latest-rust-1.75 AS cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1.75 AS builder
WORKDIR /app

# Copy the entire project context
COPY . .

# Copy cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Set sqlx offline mode to enable building without database connection
ENV SQLX_OFFLINE=true

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install SSL certificates and sqlite3
RUN apt-get update && \
    apt-get install -y ca-certificates sqlite3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/portfolio-website /app/portfolio-website

# Copy migrations folder for SQLX
COPY --from=builder /app/migrations /app/migrations

# Copy static directory
COPY --from=builder /app/static /app/static

# Create data directory and set up permissions
RUN mkdir -p /app/data && \
    # Create a specific group and user for the application
    groupadd -r appgroup && \
    useradd -r -g appgroup appuser && \
    # Give ownership to the new user
    chown -R appuser:appgroup /app && \
    # Set directory permissions
    chmod -R 755 /app && \
    chmod 777 /app/data && \
    # Create an empty database file with proper permissions
    touch /app/data/production.db && \
    chmod 666 /app/data/production.db

# Create .env file with production settings
RUN echo "DATABASE_URL=sqlite:/app/data/production.db" > /app/.env && \
    chown appuser:appgroup /app/.env

# Switch to application user
USER appuser

# Expose the port your application runs on
EXPOSE 3000

# Use JSON format for CMD to handle signals properly
CMD ["sh", "-c", "DATABASE_URL=sqlite:/app/data/production.db /app/portfolio-website"]
