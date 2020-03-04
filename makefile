.PHONY: test
test:
	cargo test -- --nocapture
.PHONY: ttest
ttest:
	RUST_BACKTRACE=1 cargo test -- --nocapture

.PHONY: doc
doc:
	cargo doc --open