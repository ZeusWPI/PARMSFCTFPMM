.PHONY: all fmt lint

all: fmt
	docker build --pull --rm -t manual_flags:latest .

fmt:
	cargo +nightly fmt

lint:
	cargo +nightly clippy
