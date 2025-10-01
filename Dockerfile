FROM rust:slim-bullseye AS builder

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the release binary
RUN apt update && apt install -y pkg-config libssl-dev ca-certificates
RUN cargo build --release

# STAGE 2
# Use a minimal Debian image 
FROM debian:bullseye-slim AS final

#Install CA certificates in the minimal runtime image
# so the running binary can validate SSL connections.
RUN apt update && apt install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/gist-api .

# Expose the required port
EXPOSE 8080

# The command to run the application
CMD ["./gist-api"]