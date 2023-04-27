FROM rust:1.69.0 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-slim as release
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/effward-dev /app/effward-dev

WORKDIR /app
COPY /src/templates /app/src/templates

EXPOSE 8080
CMD ["./effward-dev"]