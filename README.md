# effward-dev
Code for effward.dev site

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker/Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [FlyCTL](https://fly.io/docs/hands-on/install-flyctl/)

Recommended:
- [cargo-watch](https://crates.io/crates/cargo-watch)
    - `cargo install cargo-watch`
- [cargo-machete](https://github.com/bnjbvr/cargo-machete)
    - `cargo install cargo-machete`

## Format
Format code with:
```bash
cargo fmt
```

## Environment Variables
Set the following environment variables:
- DATABASE_URL (Aiven shared-sql instance)
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
- default avatar options? https://www.freepik.com/free-vector/set-linear-graphic-animal-heads_3438068.htm

# Attribution

## Image Attribution
- hero_bigfoot.svg (https://www.freepik.com/free-vector/bigfoot-concept-illustration_14204080.htm)
- hero_forest.svg (https://www.freepik.com/free-vector/forest-concept-illustration_8426448.htm)
- hero_bear-family.svg (https://www.freepik.com/free-vector/bear-family-concept-illustration_27637592.htm)
- cherry-tree.svg (https://www.freepik.com/free-vector/cherry-tree-concept-illustration_29807797.htm)
- hero_404.svg (https://www.freepik.com/free-vector/404-error-with-landscape-concept-illustration_20602802.htm)
- hero_500.svg (https://www.freepik.com/free-vector/loss-biodiversity-concept-illustration_23845297.htm)
- hero_maintenance.svg (https://www.freepik.com/free-vector/computer-trouble-shooting-concept-illustration_18771510.htm)
- hero_wip.svg (https://www.freepik.com/free-vector/software-code-testing-concept-illustration_21532465.htm)
- hero_sunny-mountains.svg (https://www.freepik.com/free-vector/snowy-mountain-concept-illustration_33804223.htm)
- hero_hiker.svg (https://www.freepik.com/free-vector/hiking-concept-illustration_8887094.htm)
- hero_kayaking.svg (https://www.freepik.com/free-vector/kayaking-concept-illustration_26233242.htm)
- hero_night-mountains.svg (https://www.freepik.com/free-vector/mountain-night-concept-illustration_33804220.htm)
- hero_town.svg (https://www.freepik.com/free-vector/small-town-concept-illustration_12516925.htm)
- hero_city.svg (https://www.freepik.com/free-vector/future-city-concept-illustration_24487818.htm)
- hero_tropical-island.svg (https://www.freepik.com/free-vector/tropical-island-concept-illustration_26233444.htm)
- hero_bear-family2.svg (https://www.freepik.com/free-vector/bear-family-illustration_27828863.htm)
- hero_aurora.svg (https://www.freepik.com/free-vector/nothern-lights-concept-illustration_34915938.htm)
