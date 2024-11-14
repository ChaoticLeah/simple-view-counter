FROM rust:latest

WORKDIR /app

RUN rm -f /var/lib/dpkg/info/ucf.md5sums && apt-get install --reinstall -y ucf

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# COPY . .

# Build the application
RUN cargo install --path .

# Expose the port your application runs on
EXPOSE 8080

# Set the startup command to run the binary
CMD ["view_counter", "--config", "/app/config.yaml", "--db", "/app/data.db"]