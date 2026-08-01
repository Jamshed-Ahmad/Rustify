FROM rust:1.80-slim as builder
WORKDIR /usr/src/rustify
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/rustify/target/release/slugify /usr/local/bin/slugify
ENTRYPOINT ["slugify"]
