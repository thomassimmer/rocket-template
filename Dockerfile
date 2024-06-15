FROM rust:latest

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features "sqlite-bundled"

COPY . .

RUN cargo build --release --locked

EXPOSE 8000

CMD ["./target/release/rocket-template"]