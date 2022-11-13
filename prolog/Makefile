.PHONY: all fmt lint

all:
	docker build --pull --rm -t manual_flags:latest .

fmt:
	cargo +nightly fmt

lint:
	cargo +nightly clippy
