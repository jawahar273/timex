.PHONY: coverage
coverage:
	cargo tarpaulin --out html \
		--output-dir ./coverage/dev \
		# --engine llvm \
		--count 

add-wasm:
	rustup target add wasm32-wasi 

wasm:
	cargo build --target wasm32-wasi

docs:
	cargo doc --open --no-deps