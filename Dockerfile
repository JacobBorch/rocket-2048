# Use the rust image for building
FROM rust:1.68.2 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code and build the project
COPY . .
COPY ./static ./static
COPY Cargo.lock .
RUN cargo build --release

# Use a newer Debian version for the runtime environment
FROM debian:bullseye-slim

# Install necessary runtime libraries
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the built binary and static files
COPY --from=builder /usr/src/app/target/release/rocket-2048 /usr/local/bin/rocket-2048
COPY --from=builder /usr/src/app/static /usr/src/app/static

# Set the working directory
WORKDIR /usr/src/app

# Expose the port used by the server
EXPOSE 8000

# Run the app
CMD ["rocket-2048"]
