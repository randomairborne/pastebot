FROM rust AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /site/

COPY --from=builder /build/target/release/website /bin/website
COPY ./resources/ ./resources/

CMD ["/bin/website"]
