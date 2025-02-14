FROM rust:1.84-slim-bullseye AS builder

RUN rustup self update

WORKDIR /app
COPY . .
RUN rustup update && cargo update
RUN cargo install --path .

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/yubihsm_connect_container .

# Build and run this container with:
# docker run -it --rm --device=/dev/bus/usb:/dev/bus/usb yubihsm_connect_container
# pass the init command to initialize
# docker run -it --rm --device=/dev/bus/usb:/dev/bus/usb yubihsm_connect_container init
ENTRYPOINT ["/app/yubihsm_connect_container"]
