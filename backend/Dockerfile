# Use the official Rust image as base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

#Copy the source code
COPY src ./src



RUN cargo build --release



#Use smaller base image for the final container
FROM debian:stable-slim AS runtime

#Install all the necessary library
RUN apt-get update \
    && apt-get install -y libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

#Set the working directory inside the container
WORKDIR /usr/src/app

#Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend .

# Copy env from builder stage
COPY --from=builder /usr/src/app/.env ./

#Expose the port to axum app runs on
EXPOSE 3000

CMD [ "./backend" ]