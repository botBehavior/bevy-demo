.PHONY: web-dev web-release native fmt check

web-dev:
	trunk serve --open --watch-stdin --release --public-url . --bin threadweaver

web-release:
	rustup target add wasm32-unknown-unknown
	trunk build --release --public-url . --bin threadweaver

native:
	cargo run -p threadweaver-launcher

fmt:
	cargo fmt

check:
	cargo check -p threadweaver-launcher
