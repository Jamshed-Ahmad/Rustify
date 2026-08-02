"""
Differential Fuzzer: Rustify vs python-slugify
Runs for 60+ seconds comparing Rust binary output vs Python for random inputs.
Saves results to fuzz/log.txt
"""
import subprocess
import random
import string
import time
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "tests", "original"))
from slugify import slugify as python_slugify

RUST_BIN = os.path.join("target", "release", "slugify.exe")
if not os.path.exists(RUST_BIN):
    RUST_BIN = os.path.join("target", "debug", "slugify.exe")

UNICODE_CHARS = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !@#$%^&*()-_=+[]{}|;:',.<>?/\\ éàüöä中文한국어日本語"

def random_input(length=None):
    if length is None:
        length = random.randint(0, 100)
    return "".join(random.choice(UNICODE_CHARS) for _ in range(length))

def rust_slugify(text):
    result = subprocess.run(
        [RUST_BIN, "--", text],
        capture_output=True, text=True, encoding="utf-8"
    )
    return result.stdout.strip()

def run_fuzzer(duration=65):
    start = time.time()
    total = 0
    divergences = 0
    log_lines = []

    log_lines.append(f"Differential Fuzzing: slugify-rs vs python-slugify")
    log_lines.append(f"Start: {time.strftime('%Y-%m-%dT%H:%M:%S')}")
    log_lines.append(f"Duration: {duration}s")
    log_lines.append("-" * 60)

    while time.time() - start < duration:
        text = random_input()
        rust_out = rust_slugify(text)
        py_out = python_slugify(text)
        total += 1

        if rust_out != py_out:
            divergences += 1
            msg = f"DIVERGE [{total}]: input={repr(text)} rust={repr(rust_out)} python={repr(py_out)}"
            log_lines.append(msg)
            print(msg)
        else:
            if total % 1000 == 0:
                elapsed = time.time() - start
                msg = f"[{elapsed:.1f}s] {total} tests run, {divergences} divergences"
                log_lines.append(msg)
                print(msg)

    elapsed = time.time() - start
    log_lines.append("-" * 60)
    log_lines.append(f"End: {time.strftime('%Y-%m-%dT%H:%M:%S')}")
    log_lines.append(f"Total: {total} inputs tested in {elapsed:.2f}s")
    log_lines.append(f"Divergences: {divergences}")
    log_lines.append(f"Result: {'PASS - Zero Divergences' if divergences == 0 else 'FAIL - Divergences Found'}")

    return log_lines, total, divergences

if __name__ == "__main__":
    if not os.path.exists(RUST_BIN):
        print(f"ERROR: Binary not found at {RUST_BIN}. Run 'cargo build --release' first.")
        sys.exit(1)
    
    duration = 65
    if len(sys.argv) > 1:
        try:
            duration = int(sys.argv[1])
        except ValueError:
            pass

    print(f"Using binary: {RUST_BIN}")
    print(f"Starting differential fuzzer for {duration} seconds...")
    log_lines, total, divergences = run_fuzzer(duration)
    with open(os.path.join("fuzz", "log.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(log_lines) + "\n")
    print(f"\nDone: {total} tests, {divergences} divergences. Log saved to fuzz/log.txt")
