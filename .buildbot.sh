#! /bin/sh

set -e

export CARGO_HOME="`pwd`/.cargo"
export RUSTUP_HOME="`pwd`/.rustup"

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
sh rustup.sh --default-host x86_64-unknown-linux-gnu --default-toolchain stable -y --no-modify-path

export PATH=`pwd`/.cargo/bin/:$PATH

rustup component add rustfmt
cargo fmt --all -- --check

cargo test
cargo test --release

cargo run --release --example dump_phdrs

# Test no_std
cargo test --no-default-features
cargo test --no-default-features --release
cargo run --no-default-features --release --example dump_phdrs
