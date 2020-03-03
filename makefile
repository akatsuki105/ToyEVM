.PHONY: test
test:
	cargo test -- --nocapture

.PHONY: doc
doc:
	cargo doc --open