FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src
COPY . . 

RUN cargo install --path .
EXPOSE 8080

CMD ["trunk", "serve", "--release"]