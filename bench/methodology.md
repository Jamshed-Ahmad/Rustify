# Benchmarking Methodology & Performance Evaluation Protocol

## Objective
Evaluate the execution throughput, startup latency, p99 tail latency, and Resident Set Size (RSS) memory footprint of the native Rust port (`slugify-rs`) against the Python original (`python-slugify`).

## Environment
* **OS**: Windows 11
* **Processor**: x86_64 multi-core CPU
* **Python Runtime**: Python 3.13.5
* **Rust Compiler**: rustc 1.97.1 (release profile with LTO enabled)

## Methodology
1. **Startup Latency**: Measure CLI instantiation overhead (`timeit` / `hyperfine`) over 100 runs.
2. **Throughput Benchmark**: Process 100,000 strings containing mixed ASCII, Unicode, HTML entities, and Cyrillic/German/Greek scripts.
3. **Memory Footprint (RSS)**: Measure peak Resident Set Size during batch string slugification.
4. **Latency Percentiles**: Capture p50, p90, p99, and max execution latency per batch.
