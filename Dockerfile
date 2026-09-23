FROM rust:latest

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-bindgen-cli --version 0.2.128

COPY . .

RUN cargo build --release

EXPOSE 3000

CMD ["/app/target/release/tda"]