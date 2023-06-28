# effward-dev
Code for effward.dev site

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker/Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [FlyCTL](https://fly.io/docs/hands-on/install-flyctl/)

Recommended:
- [cargo-watch](https://crates.io/crates/cargo-watch)
    - `cargo install cargo-watch`

## Format
Format code with:
```bash
cargo fmt
```

## Environment Variables
Set the following environment variables:
- DATABASE_URL (PlanetScale)
- HMAC_KEY (Generate 512 bit key [here](https://generate-random.org/api-key-generator/512-bit/mixed-numbers))
- REDIS_URI (redis://127.0.0.1:6379)

## Build
Build with:
```bash
docker compose build
```
OR
```bash
cargo build
```

## Run
Run with:
```bash
docker compose up
```
OR
```bash
docker compose up redis
cargo run
```
OR
```bash
docker compose up redis
cargo watch -x run
```

## Deploy
Open PR, get approved, merge. Then Github Actions will deploy.
