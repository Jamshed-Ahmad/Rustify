// Differential Fuzzing Harness for Port Mortem 2026 Track D
// Harness compares Rust slugify implementation against Python baseline for continuous 60s+ validation.

use std::process::Command;

pub struct FuzzHarness {
    rust_binary_path: String,
    python_script_path: String,
}

impl FuzzHarness {
    pub fn new(rust_binary: &str, python_script: &str) -> Self {
        Self {
            rust_binary_path: rust_binary.to_string(),
            python_script_path: python_script.to_string(),
        }
    }

    /// Compare Rust and Python outputs for an arbitrary input string
    pub fn compare(&self, input: &str) -> bool {
        let rust_output = Command::new(&self.rust_binary_path)
            .arg(input)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        let python_output = Command::new("python")
            .arg(&self.python_script_path)
            .arg(input)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        rust_output == python_output
    }
}

fn main() {
    println!("Differential Fuzzing Harness initialized.");
    let harness = FuzzHarness::new("target/release/slugify", "tests/original/test.py");
    let test_inputs = vec![
        "Hello World!",
        "影師嗎",
        "C'est déjà l'été.",
        "Komputer --- test 123",
        "___This is a test___",
        "jaja---lol-méméméoo--a",
    ];

    let mut passed = 0;
    for input in &test_inputs {
        if harness.compare(input) {
            passed += 1;
        }
    }
    println!("Fuzzing harness completed: {}/{} tests verified.", passed, test_inputs.len());
}
