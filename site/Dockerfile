FROM rust AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /site/

COPY --from=builder /build/target/release/site /bin/site
COPY ./resources/ ./resources/

CMD ["/bin/site"]
