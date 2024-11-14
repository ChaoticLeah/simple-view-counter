FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY . .

# Build the application
RUN cargo install --path .

# Expose the port your application runs on
EXPOSE 8080

# Set the startup command to run the binary
CMD ["view_counter"]
# CMD ["view_counter", "--config", "/app/config.toml"]