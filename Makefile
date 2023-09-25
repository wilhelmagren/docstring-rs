.PHONY: clean
clean:
	cargo clean

.PHONY: debug 
debug:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: check
check:
	cargo check

.PHONY: format
format:
	cargo fmt --all -- --check

.PHONY: lint
lint:
	cargo clippy --all-features

.PHONY: coverage
coverage:
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
