.PHONY: help web desktop tui build build-web build-desktop build-tui
.PHONY: check fmt fmt-check lint test check-core-wasm check-tui clean
.PHONY: android ios build-android build-ios

# The Tauri targets share Trunk's dist/ and development port.
.NOTPARALLEL:

help:
	@echo "make web | desktop | tui          Run one development interface"
	@echo "make android | ios               Run on mobile devices or simulators"
	@echo "make build                       Build Web, Desktop, and Terminal"
	@echo "make build-web | build-desktop | build-tui"
	@echo "make build-android | build-ios   Build a mobile release package"
	@echo "make check                       Run the local quality checks"
	@echo "make fmt | fmt-check | lint | test | check-core-wasm | check-tui"
	@echo "make clean                       Remove Trunk and Cargo build outputs"

web:
	trunk serve

desktop:
	cargo tauri dev

tui:
	cargo run --manifest-path tui/Cargo.toml

android:
	cargo tauri android dev $(ANDROID_ARGS)

ios:
	cargo tauri ios dev $(IOS_ARGS)

# Tauri builds the production web bundle before packaging the desktop app.
build: build-desktop build-tui

build-web:
	trunk build --release

build-desktop:
	cargo tauri build -- --locked

build-tui:
	cargo build --locked --release --manifest-path tui/Cargo.toml

build-android:
	cargo tauri android build $(ANDROID_ARGS) -- --locked

build-ios:
	cargo tauri ios build $(IOS_ARGS) -- --locked

check: fmt-check lint test check-core-wasm check-tui

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --locked -p app-core --all-targets -- -D warnings
	cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings
	cargo clippy --locked --manifest-path tui/Cargo.toml --all-targets -- -D warnings
	cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings

test:
	cargo test --locked -p app-core

check-core-wasm:
	cargo check --locked -p app-core --target wasm32-unknown-unknown

check-tui:
	cargo check --locked --manifest-path tui/Cargo.toml

clean:
	trunk clean
	cargo clean
