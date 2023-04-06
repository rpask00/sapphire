
# Create a new, smaller image
FROM debian:buster-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev && \
    apt install glibc-source -y && \
    rm -rf /var/lib/apt/lists/*




# Use a Rust base image
FROM rust:latest as builder
# Set the working directory
WORKDIR /app
# Copy the rest of the source code
COPY . .

# Build the application
RUN cargo build --release


# Copy the built binary from the previous stage

# Expose a port (if needed)
EXPOSE 8080

#CMD ls -la

# Set the command to run when the container starts
CMD ["./target/release/rusty_sapphire"]
