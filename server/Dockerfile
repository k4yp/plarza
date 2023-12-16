FROM rust:latest

COPY .env .
COPY ./server .

RUN cargo build --release

CMD ["./target/release/server"]