FROM ekidd/rust-musl-builder:stable as build
COPY Cargo.toml ./
RUN mkdir .cargo/ && cargo vendor > .cargo/config
COPY . .
RUN cargo install --path . --verbose

FROM alpine
ENV RUST_LOG=info
# Don't run as root
USER 523
COPY --from=build /home/rust/.cargo/bin/oxilink .
CMD oxilink
