# Stage 1: Build the project
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build the dependencies only (to cache this layer)
RUN cargo build --release

# Stage 2: Create a smaller image for running the binary
FROM debian:latest

RUN apt update && apt install -y mariadb-client

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/kniffel /usr/local/bin/kniffel

# Set the entrypoint to the compiled binary
ENTRYPOINT ["kniffel"]

# (Optional) Set the command line arguments
CMD []
