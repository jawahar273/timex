CORE_PATH="./core"


install:
	echo "demo required, rust(https://github.com/rust-lang/rust), golang(https://github.com/golang/go) and nvm(https://github.com/nvm-sh/nvm)";
	echo "install rust"
	 cargo build
	echo "install golang"
	go mod tidy
	echo "install web"
	npm install

	
r.app:
	cargo run

g.app:
	go run main.go

.PHONY: web
web:
	npm run dev

# TODO: on git push make sure run this command
readme:
	cp ./README.md ${CORE_PATH}

go-grpc:
	protoc  \
		--go_out=./ --go_opt=paths=source_relative \
		--go-grpc_out=. --go-grpc_opt=paths=source_relative \
		--proto_path=. \
		./proto/api/v1/*.proto

# asm:
# rm -rf ./web/pkg
# cd ${CORE_PATH} && \
# wasm-pack build --target web && \
# cd .. && \
# mv ${CORE_PATH}/pkg ./server/web/
	
