.PHONY: run
run:
	cargo run

.PHONY: test
test:
	cargo test
.PHONY: ttest
ttest:
	cargo test -- --nocapture
.PHONY: tttest
tttest:
	RUST_BACKTRACE=1 cargo test -- --nocapture

.PHONY: doc
doc:
	cargo doc --open