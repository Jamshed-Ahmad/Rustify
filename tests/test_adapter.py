import sys
import os
import subprocess
import unittest

RUST_BIN = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "release", "slugify.exe"))
if not os.path.exists(RUST_BIN):
    RUST_BIN = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "slugify.exe"))

ENV = os.environ.copy()
MINGW_BIN = r"C:\Users\Lenovo\AppData\Local\Microsoft\WinGet\Packages\MartinStorsjo.LLVM-MinGW.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\llvm-mingw-20260616-ucrt-x86_64\bin"
if os.path.exists(MINGW_BIN):
    ENV["PATH"] = MINGW_BIN + ";" + ENV.get("PATH", "")

def rust_slugify(
    text,
    entities=True,
    decimal=True,
    hexadecimal=True,
    max_length=0,
    word_boundary=False,
    separator='-',
    save_order=False,
    stopwords=(),
    regex_pattern=None,
    lowercase=True,
    replacements=(),
    allow_unicode=False,
):
    cmd = [RUST_BIN]

    if not entities:
        cmd.append("--no-entities")
    if not decimal:
        cmd.append("--no-decimal")
    if not hexadecimal:
        cmd.append("--no-hexadecimal")
    if max_length > 0:
        cmd.extend(["--max-length", str(max_length)])
    if word_boundary:
        cmd.append("--word-boundary")
    if save_order:
        cmd.append("--save-order")
    if separator != '-':
        cmd.extend(["--separator", separator])
    if stopwords:
        cmd.append("--stopwords")
        cmd.extend(list(stopwords))
    if regex_pattern:
        cmd.extend(["--regex-pattern", str(regex_pattern)])
    if not lowercase:
        cmd.append("--no-lowercase")
    if replacements:
        for old_val, new_val in replacements:
            cmd.extend(["--replacements", f"{old_val}->{new_val}"])
    if allow_unicode:
        cmd.append("--allow-unicode")

    cmd.append("--")
    cmd.append(str(text))

    res = subprocess.run(cmd, capture_output=True, text=True, encoding='utf-8', env=ENV)
    return res.stdout.strip()

original_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "original"))
sys.path.insert(0, original_dir)

import slugify as python_slugify_module
python_slugify_module.slugify = rust_slugify

if __name__ == "__main__":
    import test as original_test_suite
    print("[Adapter] Executing original test.py test suite against Rust binary...")
    suite = unittest.TestLoader().loadTestsFromModule(original_test_suite)
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    sys.exit(not result.wasSuccessful())
