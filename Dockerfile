FROM rust:1.69.0

EXPOSE 8080

WORKDIR /app
COPY . .

RUN openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'

RUN cargo install --path .
CMD ["effward-dev"]