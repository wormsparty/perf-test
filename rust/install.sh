#!/bin/sh

if ! which cargo > /dev/null 2>&1; then
	curl https://sh.rustup.rs -sSf | sh
fi

cargo install cargo-watch

# Build
rustup default stable
rustup update
cargo b -r

# Deploy
cp target/release/rust-test .
echo "Executable available at ./rust-test"
