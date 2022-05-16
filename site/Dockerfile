FROM rust AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

FROM debian:stable-slim

COPY --from=builder /build/target/release/website /bin/website

CMD ["/bin/website"]