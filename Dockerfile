# Use the latest Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the project files into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a newer base image with updated glibc
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rustbook .

# Copy the static directory into the container
COPY static ./static

# Expose the port your application runs on
EXPOSE 8080

# Set the command to run the application
CMD ["./rustbook"]