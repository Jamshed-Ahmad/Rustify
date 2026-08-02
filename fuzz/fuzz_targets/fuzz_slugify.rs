#![no_main]

use libfuzzer_sys::fuzz_target;
use slugify::{slugify, SlugifyOptions};

fuzz_target!(|data: &[u8]| {
    // Only process valid UTF-8 input
    if let Ok(s) = std::str::from_utf8(data) {
        let opts = SlugifyOptions::default();
        let _ = slugify(s, &opts);
    }
});
