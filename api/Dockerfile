FROM rust AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

FROM debian:slim

COPY --from=builder /build/target/release/api /bin/api

EXPOSE 8080

CMD ["/bin/api"]