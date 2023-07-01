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

# `TODO:`

## Bulma Ideas
### Bulma Extensions
https://bulma.io/extensions/

- bulma-carousel (image posts, project showcase)
- bulma-iconpicker (post icon (category) picker)
- bulma-tagsinput (tagging posts)
- bulma-ribbon / bulma-badge (notification counters)
- bulma-timeline (alternate resume view?)
- bulma-toast (make notifications float)

### Vector images
- @storyset on freepik: https://www.freepik.com/author/stories

# Attribution

- hero_bigfoot.svg (https://www.freepik.com/free-vector/bigfoot-concept-illustration_14204080.htm#page=2&position=33&from_view=author)
- hero_forest.svg (https://www.freepik.com/free-vector/forest-concept-illustration_8426448.htm#page=2&position=12&from_view=author)
- hero_bear-family.svg (https://www.freepik.com/free-vector/bear-family-concept-illustration_27637592.htm#from_view=detail_alsolike)
- cherry-tree.svg (https://www.freepik.com/free-vector/cherry-tree-concept-illustration_29807797.htm#&position=2&from_view=author)