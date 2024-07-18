set windows-shell := ["pwsh.exe", "-Command"]

init:
	cargo install cargo-binstall
	cargo binstall cargo-insta cargo-nextest cargo-wasi cargo-release

build:
	cargo wasi build

check:
	cargo check --workspace

format:
	cargo fmt --all

format-check:
	cargo fmt --all --check

lint:
	cargo clippy --workspace --all-targets

test name="":
	just build
	cargo nextest run --workspace {{name}}
