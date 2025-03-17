# Use the official Rust image as the base image
FROM rust:1.49-slim-buster

# Set the working directory inside the container
WORKDIR /usr/src/app

# Accept the Rust source file name as a build argument
ARG RUST_FILE

# Copy the specified Rust source file into the container
COPY ${RUST_FILE} ./main.rs

# Build the Rust source file
RUN rustc main.rs

# Set the command to run the compiled binary
CMD ["./main"]