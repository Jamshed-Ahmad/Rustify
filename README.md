# Rustify (`slugify-rs`)

Idiomatic, high-performance, 100% safe Rust port of `python-slugify`.

## Build & Run

### Native Build
```bash
cargo build --release
./target/release/slugify "Hello World! This is Rustify."
```

### Docker Build
```bash
docker build -t rustify .
docker run --rm rustify "Hello World! This is Rustify."
```

## Verification & Testing
- **Native Rust Tests**: `cargo test`
- **Original Test Adapter**: `python tests/test_adapter.py`
- **Differential Fuzzing**: `cargo run --bin harness`
