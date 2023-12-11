.PHONY: 2023 cli api
2023:
	cargo build

cli:
	cargo run -p cli

api:
	cargo run -p api

test:
	cargo test

