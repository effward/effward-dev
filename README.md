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

## Build
Build with:
```bash
docker build -t effward-dev .
```
OR
```bash
cargo build
```

## Run
Run with:
```bash
docker run -it --rm -p 8080:8080 effward-dev
```
OR
```bash
cargo run
```
OR
```bash
cargo watch -x run
```

## Deploy
Open PR, get approved, merge. Then Github Actions will deploy.
