FROM ekidd/rust-musl-builder AS builder
ADD Cargo.toml ./
ADD src ./src
RUN sudo chown -R rust:rust /home/rust/src
RUN cargo build --release
RUN strip /home/rust/src/target/x86_64-unknown-linux-musl/release/ap_firmware_bot

FROM scratch AS runner
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/ap_firmware_bot \
  /ap_firmware_bot

ENTRYPOINT [ "/ap_firmware_bot" ]