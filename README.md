# effward-dev
Code for effward.dev site

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker/Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [FlyCTL](https://fly.io/docs/hands-on/install-flyctl/)

## Build
Build with:
```bash
docker build -t effward-dev .
```

## Run
Run with:
```bash
docker run -it --rm -p 8080:8080 effward-dev
```

## Deploy
Open PR, get approved, merge. Then Github Actions will deploy.
