FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

RUN cargo install --path .
EXPOSE 8080

CMD ["trunk", "serve", "--release"]