FROM rust:1-alpine3.21 AS builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/app

CMD ["cargo", "build", "release"]



