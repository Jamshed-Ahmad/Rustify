# slugify-rs (`Rustify`)

**Port Mortem 2026 · Track D: Python → Rust**

A high-performance, standalone Rust port of [`python-slugify`](https://github.com/un33k/python-slugify) — completely independent of the Python runtime, with zero FFI calls and zero unsafe code.

---

## Track Alignment

| Field | Value |
|---|---|
| **Track** | D (Python → Rust) |
| **Source** | `https://github.com/un33k/python-slugify.git` |
| **Strategy** | Astral Playbook — idiomatic Rust rewrite with CLI test bridge |
| **Runtime Dependency on Python** | ❌ None |
| **FFI / C Extensions** | ❌ None |
| **Unsafe Blocks** | ✅ **0** (`#![forbid(unsafe_code)]` on every module) |

---

## ⭐ The North Star: Behavioral Equivalence

The original `python-slugify` test suite (`tests/original/test.py`) runs **unmodified** against the compiled Rust binary via a thin Python adapter.

```
Ran 82 tests in 5.490s

OK
```

| Metric | Result |
|---|---|
| **Test Pass Rate** | **100% (82/82)** |
| **Original tests modified** | ❌ None |
| **Adapter approach** | `tests/test_adapter.py` intercepts `slugify.slugify` and delegates to the Rust CLI binary |

---

## 🚀 Performance at a Glance

| Metric | Python (`python-slugify`) | Rust (`slugify-rs`) | Improvement |
|---|---|---|---|
| **Startup Time** | 38.5 ms | 1.2 ms | **32× faster** |
| **Throughput** | 42,500 ops/sec | 603,500 ops/sec | **14.2× faster** |
| **p99 Latency** | 23.5 µs | 1.65 µs | **14.2× lower** |
| **Memory RSS** | 24.8 MB | 2.1 MB | **11.8× smaller** |

> Measured on Windows 11, x86\_64, rustc 1.97.1 (release profile, LTO enabled).
> Full methodology documented in [`bench/methodology.md`](bench/methodology.md).

---

## 🔒 Engineering Bonuses

### Zero Unsafe

Every Rust source file enforces the compiler-level guarantee:

```rust
#![forbid(unsafe_code)]
```

Confirmed across: `src/lib.rs`, `src/slugify.rs`, `src/special.rs`, `src/main.rs`, `src/error.rs`.

### Differential Fuzz Survivor

A Python-based differential fuzzer fed 1,407 random Unicode strings simultaneously to both implementations over **65 continuous seconds**.

```
Total: 1407 inputs tested in 65.00s
Divergences: 0
Result: PASS - Zero Divergences
```

Full run log saved at [`fuzz/log.txt`](fuzz/log.txt).

### Engineering Discipline

[`DECISIONS.md`](DECISIONS.md) documents **10 non-trivial architectural divergences**, including:

- `Cow<'a, str>` vs CPython `str` allocation model
- `once_cell::sync::Lazy` pre-compiled regex vs Python `re._cache`
- `u32::from_str_radix` + `char::from_u32()` vs Python `try/except` entity parsing
- `HashSet<String>` stopwords for O(1) lookup vs Python list O(N) scan
- Static transliteration tables (`deunicode`) vs Python dynamic `import unidecode`

---

## 📁 Repository Map

```
Rustify/
├── .port-mortem.toml       # Track D metadata & kickoff hashes
├── Cargo.toml              # Rust build manifest
├── Dockerfile              # Single-command reproducible build
├── DECISIONS.md            # 10 architectural divergences
├── bench/
│   ├── methodology.md      # Benchmarking environment & commands
│   └── results.json        # p99, RSS, startup measurements
├── fuzz/
│   ├── differential_fuzz.py  # 65s differential fuzzer
│   ├── harness.rs            # Structural fuzzing harness
│   ├── fuzz_targets/
│   │   └── fuzz_slugify.rs   # cargo-fuzz target
│   └── log.txt               # ✅ 1407 tests · 0 divergences
├── src/
│   ├── lib.rs              # Public API surface
│   ├── slugify.rs          # Core slugification logic
│   ├── special.rs          # Transliteration maps (Cyrillic, German, Greek)
│   ├── error.rs            # Native Rust error types
│   └── main.rs             # CLI binary & test adapter bridge
└── tests/
    ├── test_adapter.py     # Python bridge: original tests → Rust binary
    └── original/           # Pinned, unmodified python-slugify test suite
```

---

## 🔧 Build & Run

### Native (single command)

```bash
cargo build --release
./target/release/slugify "Hello, World! Это тест."
# → hello-world-eto-test
```

### Docker (reproducible, no local Rust required)

```bash
docker build -t rustify .
docker run --rm rustify "Hello, World! Это тест."
```

### Run Original Test Suite

```bash
python tests/test_adapter.py
# Ran 82 tests in 5.490s  OK
```

### Run Differential Fuzzer

```bash
python fuzz/differential_fuzz.py
# Done: 1407 tests, 0 divergences. Log saved to fuzz/log.txt
```

---

## Migration Rationale

`python-slugify` is used in high-throughput web pipelines (URL normalization, content indexing) where CPython's GIL, startup overhead, and object allocation patterns create measurable latency under load.

Rust was chosen for:

- **Predictable latency**: No garbage collector pauses; deterministic allocation via stack and `Cow<str>`.
- **Memory safety**: The compiler statically eliminates buffer overflows, use-after-free, and data races — with zero runtime cost.
- **Startup speed**: 1.2 ms vs 38.5 ms — critical for CLI invocation patterns in CI/CD pipelines.
- **ReDoS resistance**: The `regex` crate uses finite automata with guaranteed O(N) execution; Python's `re` module is vulnerable to catastrophic backtracking.

For all 10 architectural decisions, see [`DECISIONS.md`](DECISIONS.md).

---

## License

MIT — Same as the original `python-slugify`.
