
install:
	echo "demo required, rust(https://github.com/rust-lang/rust), golang(https://github.com/golang/go) and nvm(https://github.com/nvm-sh/nvm)";
	echo "install rust"
	cd ./server/ && cargo build
	echo "install golang"
	cd ./server/ && go mod tidy
	echo "install web"
	cd ./server/web && nvm use && npm install

r.app:
	cd ./server/ &&\
	cargo run

g.app:
	cd ./server/ &&\
	go run main.go

web:
	cd ./server/web &&\
	npm run dev