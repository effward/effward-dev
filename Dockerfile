FROM rust:1.69.0 as builder

ARG DB_URL

ENV DATABASE_URL=${DB_URL}

WORKDIR /app
COPY . .

RUN cargo build --release

WORKDIR /app/target/release
COPY /templates /app/target/release/templates
COPY /public /app/target/release/public

ENV DATABASE_URL=

EXPOSE 8080
CMD ["./effward_dev"]
