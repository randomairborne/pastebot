FROM rust AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

FROM debian:stable-slim

COPY --from=builder /build/target/release/web /bin/web

EXPOSE 8080

CMD ["/bin/web"]