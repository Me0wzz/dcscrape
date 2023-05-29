FROM rust:latest AS builder
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim


ENV GALLERY programming
ENV INTERVAL 600
ENV UID 1001
ENV GID 1001
ENV TZ Asia/Seoul

RUN apt-get update && apt install -y openssl ca-certificates
COPY --from=builder ./target/release/dcscrape ./
CMD while true; do ./dcscrape $GALLERY; sleep $INTERVAL; done