FROM rust:latest
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install -f wasm-bindgen-cli

