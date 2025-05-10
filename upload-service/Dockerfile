FROM rust:latest

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src

RUN cargo build --release
RUN rm -drf src

EXPOSE 8000
CMD ["/app/target/release/upload-service"]
