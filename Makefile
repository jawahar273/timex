.PHONY: coverage release wasm init pre-release

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

init-dep:
	cargo install cargo-audit \
		cargo-tarpaulin \
		cargo-release \
		git-cliff
	# cargo install flamegraph

pre-release:
	@test $${version?Please set the version for release}
	git cliff -o CHANGELOG.md --tag $(version)
	cargo release version $(version) --execute --no-confirm

relase:
	cargo release commit --execute
	cargo release tag --execute --no-confirm

ASM_PATH="./asm"
WEB_PATH="./server/web"
WEB_ASM_PATH="${WEB_PATH}/src/asm"

COMMIT_ID=$(shell git rev-parse --short HEAD)
ASM_FILE_NAME="asm-${COMMIT_ID}"

js-asm:
	echo "version: ${COMMIT_ID}" && \
	cd ${ASM_PATH} && \
	rm -rf "${ASM_PATH}/pkg" && \
	wasm-pack build --target web --out-name ${ASM_FILE_NAME} && \
	cd .. && \
	rm -rf "${WEB_ASM_PATH}/*" && \
	cp -a ${ASM_PATH}/pkg/* ${WEB_ASM_PATH} && \
	echo "export const VERSION = \"${COMMIT_ID}\"" | cat > "${WEB_ASM_PATH}/version.ts"
	

ci-install:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	