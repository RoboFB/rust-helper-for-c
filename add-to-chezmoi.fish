#!/usr/bin/env fish

cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/rust-helper-for-c ~/.local/bin/rust-helper-for-c
chezmoi add ~/.local/bin/rust-helper-for-c