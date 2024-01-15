FROM rust:1.72.0 as rust-build

RUN apt update && apt install -y protobuf-compiler

WORKDIR /app

COPY ./ /app

# RUN protoc --prost-serde_out=/app/server/proto/ -I proto  /app/server/proto/api/v1/glue.proto
RUN cd /app/server && cargo build --release
RUN mv /app/server/target/release/server /app/server/target/release/app

FROM golang:1.21.5 as go-build

WORKDIR /go/src/app

RUN apt update && apt install -y protobuf-compiler

COPY ./server ./

RUN go mod download
RUN go vet -v
RUN go test -v

RUN CGO_ENABLED=0 go build -o /go/bin/app


# FROM  gcr.io/distroless/static-debian12
FROM debian
WORKDIR /demo

COPY --from=rust-build /app/server/target/release/app /demo/r/
COPY --from=go-build /go/bin/app /demo/g/

