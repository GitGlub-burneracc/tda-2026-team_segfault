FROM rust:latest

WORKDIR /app

COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-bindgen-cli

RUN cargo build --release

EXPOSE 3000

CMD ["/app/target/release/tda"]