FROM rust AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

FROM debian:slim

COPY --from=builder /build/target/release/bot /bin/bot

CMD ["/bin/bot"]