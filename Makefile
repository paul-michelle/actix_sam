test:
	cargo test -- --nocapture

check:
	@cargo fmt --all
	@cargo clippy --no-deps -- -D warnings

serve: check
	fuser -k 7878/tcp || true && cargo run

coverage:
	cargo tarpaulin --ignore-tests --avoid-cfg-tarpaulin

audit:
	cargo audit

lint:
	cargo fmt --all --check && cargo clippy --no-deps -- -D warnings
