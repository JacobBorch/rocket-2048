FROM rust:1.68.2 as builder

WORKDIR /usr/src/app

COPY . .
COPY ./static ./static
COPY Cargo.lock .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rocket-2048 /usr/local/bin/rocket-2048
COPY --from=builder /usr/src/app/static /usr/src/app/static

WORKDIR /usr/src/app

EXPOSE 8000

CMD ["rocket-2048"]
