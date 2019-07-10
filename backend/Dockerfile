FROM rust:1.36.0-stretch

WORKDIR /project
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release
