# crabfetch 🦀

A simple fetch utility, written in Rust

[![forthebadge](https://forthebadge.com/images/badges/works-on-my-machine.svg)](https://forthebadge.com)

![GitHub](https://img.shields.io/github/license/flyingcakes85/crabfetch?style=for-the-badge)

[![forthebadge](https://forthebadge.com/images/badges/built-with-love.svg)](https://forthebadge.com) [![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

![Screenschot](screenshot.png)

## Install

### AUR (Arch and Arch Based)

Install [`crabfetch`](https://aur.archlinux.org/packages/crabfetch) from AUR.

### Manual

[![Rust](https://github.com/flyingcakes85/crabfetch/actions/workflows/rust.yml/badge.svg)](https://github.com/flyingcakes85/crabfetch/actions/workflows/rust.yml)

1. Get the repo

```sh
git clone https://github.com/flyingcakes85/crabfetch
cd crabfetch
```

2. Build it!

```sh
cargo build --release
```

3. Copy the binary

- For current user (root not required)
```sh
cp target/release/crabfetch ~/.local/bin/crabfetch
```

## License

GNU GPL v3. [Text Here](https://github.com/flyingcakes85/crabfetch/blob/main/COPYING)
