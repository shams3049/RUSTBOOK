# Build stage
FROM rust:1.62 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# Runtime stage
FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/rustbook /usr/local/bin/rustbook

# Create a non-root user to run the application
RUN groupadd -r appuser && useradd -r -g appuser appuser
USER appuser

EXPOSE 8080

CMD ["rustbook"]
