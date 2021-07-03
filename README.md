### Galaxy-rs
[![GitHub issues](https://img.shields.io/github/issues/dzosz/Galaxy-rs)](https://github.com/dzosz/Galaxy-rs/issues)
[![GitHub watchers](https://img.shields.io/github/watchers/dzosz/Galaxy-rs?style=social&label=Watch&maxAge=2592000)](https://github.com/dzosz/Galaxy-rs/watchers/)
[![GitHub code size](https://img.shields.io/github/languages/code-size/dzosz/Galaxy-rs?style=flat)](https://github.com/dzosz/Galaxy-rs)
## About
__This project allows you to run a simulation of the collision of two galaxies. The project was based on the law of universal gravitation.__<br>
**Project use Rust 2018 edition

## Demonstration
[Demo animation](https://www.youtube.com/watch?v=x62gOfZ9hCw&feature=emb_logo)

## How use
Build all
```bash
cargo build --release
```

Start with 3 Body
```bash
cargo run --release --bin threebody
```

Start with  Sun Earth Moon system
```bash
cargo run --release --bin sun_earth_moon
```

Start Collision
```bash
cargo run --release --bin collision
```

Clean project
```bash
cargo clean
```
