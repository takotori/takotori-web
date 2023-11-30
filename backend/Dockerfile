FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /api
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /api/target/x86_64-unknown-linux-musl/release/takotori-web /api
ENTRYPOINT ["/api"]
EXPOSE 8000