coverage:
	cargo tarpaulin --out html --output-dir ./coverage --engine llvm

add-wasm:
	rustup target add wasm32-wasi 

wasm:
	cargo build --target wasm32-wasi
	