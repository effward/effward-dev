FROM rust:1.69.0 as builder

WORKDIR /app
COPY . .

RUN cargo install --path .

FROM debian:buster-slim as release
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/effward-dev /usr/local/bin/effward-dev

EXPOSE 8080
CMD ["effward-dev"]