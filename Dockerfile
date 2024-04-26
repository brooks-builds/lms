FROM rust:1.77

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

VOLUME /code/lms

WORKDIR /code/lms

COPY ./ /code/lms

EXPOSE 8080

CMD ["trunk", "serve", "--address", "0.0.0.0"]
