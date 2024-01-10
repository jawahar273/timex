

go-grpc:
	protoc  \
		--go_out=./ --go_opt=paths=source_relative \
		--go-grpc_out=. --go-grpc_opt=paths=source_relative \
		--proto_path=. \
		./proto/api/v1/*.proto

asm:
	rm -rf ./web/pkg
	cd ../core && \
	wasm-pack build --target web && \
	cd .. && \
	mv ./core/pkg ./server/web/