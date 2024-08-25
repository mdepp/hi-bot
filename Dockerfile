FROM rust:1-alpine
RUN apk update && apk add g++
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/main.rs src/
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
WORKDIR /app
COPY --from=0 /app/target/x86_64-unknown-linux-musl/release/hi-bot .
RUN touch .env
CMD ["/app/hi-bot"]
