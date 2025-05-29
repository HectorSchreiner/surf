FROM rust:1.87-alpine3.21 AS build
WORKDIR /app

RUN apk add musl-dev openssl-dev

# Building dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --locked --release --features=docs

# Building application
COPY . .
RUN touch src/main.rs
RUN cargo build --locked  --release --features=docs
RUN ls /app/target/release

FROM alpine:3.21
WORKDIR /app
COPY --from=build /app/target/release/surf .
ENTRYPOINT ["./surf"]