# Builder image
FROM rust:alpine AS builder
RUN apk update
RUN apk add --no-cache openssl-dev musl-dev

WORKDIR /code
ADD Cargo.toml ./
ADD src ./src
RUN cargo build --release

RUN ls -a -R /code
RUN strip /code/target/release/ap_firmware_bot

# Runner image
FROM alpine:latest AS runner
COPY --from=builder /code/target/release/ap_firmware_bot \
  /ap_firmware_bot

ENTRYPOINT [ "/ap_firmware_bot" ]