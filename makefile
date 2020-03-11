ifdef COMSPEC
	EXE_EXT := .exe
else
	EXE_EXT := 
endif

.PHONY: build
build:
	rm -rf toyevm$(EXE_EXT)
	cargo build --release
	mv ./target/release/toyevm$(EXE_EXT) .

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

.PHONY: clean
clean:
	rm -rf toyevm$(EXE_EXT)