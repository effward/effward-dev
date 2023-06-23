FROM rust:1.69.0 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

WORKDIR /app/target/release
COPY /static /app/target/release/static

EXPOSE 8080
CMD ["./effward-dev"]
