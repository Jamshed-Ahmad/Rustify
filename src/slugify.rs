#![forbid(unsafe_code)]

use deunicode::deunicode;
use html_escape::decode_html_entities;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use unicode_normalization::UnicodeNormalization;

pub const DEFAULT_SEPARATOR: &str = "-";

static QUOTE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\']+").unwrap());
static DISALLOWED_CHARS_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^-a-zA-Z0-9]+").unwrap());
static DISALLOWED_UNICODE_CHARS_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\W_]+").unwrap());
static DUPLICATE_DASH_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"-{2,}").unwrap());
static NUMBERS_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d),(\d)").unwrap());
static NAMED_ENTITY_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"&([a-zA-Z]+);").unwrap());
static DECIMAL_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"&#(\d+);").unwrap());
static HEX_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"&#x([\da-fA-F]+);").unwrap());

/// Options struct for slugify customization
#[derive(Debug, Clone)]
pub struct SlugifyOptions<'a> {
    pub entities: bool,
    pub decimal: bool,
    pub hexadecimal: bool,
    pub max_length: usize,
    pub word_boundary: bool,
    pub separator: &'a str,
    pub save_order: bool,
    pub stopwords: Vec<String>,
    pub regex_pattern: Option<String>,
    pub lowercase: bool,
    pub replacements: Vec<(String, String)>,
    pub allow_unicode: bool,
}

impl<'a> Default for SlugifyOptions<'a> {
    fn default() -> Self {
        Self {
            entities: true,
            decimal: true,
            hexadecimal: true,
            max_length: 0,
            word_boundary: false,
            separator: DEFAULT_SEPARATOR,
            save_order: false,
            stopwords: Vec::new(),
            regex_pattern: None,
            lowercase: true,
            replacements: Vec::new(),
            allow_unicode: false,
        }
    }
}

/// Helper to get slice of first N unicode characters
fn take_chars(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

/// Helper to strip leading and trailing separator occurrences
fn strip_separator<'a>(s: &'a str, sep: &str) -> &'a str {
    let mut res = s;
    while res.starts_with(sep) && !sep.is_empty() {
        res = &res[sep.len()..];
    }
    while res.ends_with(sep) && !sep.is_empty() {
        res = &res[..res.len() - sep.len()];
    }
    res
}

pub fn smart_truncate(
    string: &str,
    max_length: usize,
    word_boundary: bool,
    separator: &str,
    save_order: bool,
) -> String {
    let s = strip_separator(string, separator);

    if max_length == 0 {
        return s.to_string();
    }

    let char_count = s.chars().count();
    if char_count < max_length {
        return s.to_string();
    }

    if !word_boundary {
        let sliced = take_chars(s, max_length);
        return strip_separator(sliced, separator).to_string();
    }

    if !s.contains(separator) {
        return take_chars(s, max_length).to_string();
    }

    let mut truncated = String::new();
    for word in s.split(separator) {
        if !word.is_empty() {
            let trunc_len = truncated.chars().count();
            let word_len = word.chars().count();
            let next_len = trunc_len + word_len;

            if next_len < max_length {
                truncated.push_str(word);
                truncated.push_str(separator);
            } else if next_len == max_length {
                truncated.push_str(word);
                break;
            } else {
                if save_order {
                    break;
                }
            }
        }
    }

    if truncated.is_empty() {
        truncated = take_chars(s, max_length).to_string();
    }

    strip_separator(&truncated, separator).to_string()
}

pub fn slugify(text: &str, options: &SlugifyOptions) -> String {
    let mut text_buf = text.to_string();

    // 1. Initial user replacements
    for (old_val, new_val) in &options.replacements {
        text_buf = text_buf.replace(old_val, new_val);
    }

    // 2. Quote pattern pre-process
    text_buf = QUOTE_PATTERN.replace_all(&text_buf, DEFAULT_SEPARATOR).to_string();

    // 3. Unicode normalization / transliteration
    if options.allow_unicode {
        text_buf = text_buf.nfkc().collect::<String>();
    } else {
        let nfkd = text_buf.nfkd().collect::<String>();
        text_buf = deunicode(&nfkd);
    }

    // 4. HTML Character Entities (Named entities only)
    if options.entities {
        text_buf = NAMED_ENTITY_PATTERN
            .replace_all(&text_buf, |caps: &regex::Captures| {
                decode_html_entities(&caps[0]).to_string()
            })
            .to_string();
    }

    // 5. Decimal character references
    if options.decimal {
        text_buf = DECIMAL_PATTERN
            .replace_all(&text_buf, |caps: &regex::Captures| {
                if let Ok(code) = caps[1].parse::<u32>() {
                    if let Some(ch) = std::char::from_u32(code) {
                        return ch.to_string();
                    }
                }
                caps[0].to_string()
            })
            .to_string();
    }

    // 6. Hexadecimal character references
    if options.hexadecimal {
        text_buf = HEX_PATTERN
            .replace_all(&text_buf, |caps: &regex::Captures| {
                if let Ok(code) = u32::from_str_radix(&caps[1], 16) {
                    if let Some(ch) = std::char::from_u32(code) {
                        return ch.to_string();
                    }
                }
                caps[0].to_string()
            })
            .to_string();
    }

    // 7. Re-normalize text
    if options.allow_unicode {
        text_buf = text_buf.nfkc().collect::<String>();
    } else {
        text_buf = text_buf.nfkd().collect::<String>();
    }

    // 8. Lowercase
    if options.lowercase {
        text_buf = text_buf.to_lowercase();
    }

    // 9. Remove generated quotes post-process
    text_buf = QUOTE_PATTERN.replace_all(&text_buf, "").to_string();

    // 10. Clean up numbers
    while NUMBERS_PATTERN.is_match(&text_buf) {
        text_buf = NUMBERS_PATTERN.replace_all(&text_buf, "${1}${2}").to_string();
    }

    // 11. Disallowed characters filtering
    let custom_regex = options
        .regex_pattern
        .as_ref()
        .and_then(|pat| Regex::new(pat).ok());

    let text_replaced = match custom_regex {
        Some(re) => re.replace_all(&text_buf, DEFAULT_SEPARATOR).to_string(),
        None => {
            if options.allow_unicode {
                DISALLOWED_UNICODE_CHARS_PATTERN
                    .replace_all(&text_buf, DEFAULT_SEPARATOR)
                    .to_string()
            } else {
                DISALLOWED_CHARS_PATTERN
                    .replace_all(&text_buf, DEFAULT_SEPARATOR)
                    .to_string()
            }
        }
    };

    // 12. Remove duplicate dash & strip default separator
    let mut cleaned = DUPLICATE_DASH_PATTERN
        .replace_all(&text_replaced, DEFAULT_SEPARATOR)
        .to_string();
    cleaned = strip_separator(&cleaned, DEFAULT_SEPARATOR).to_string();

    // 13. Stopwords removal
    if !options.stopwords.is_empty() {
        let stopwords_set: HashSet<String> = if options.lowercase {
            options.stopwords.iter().map(|s| s.to_lowercase()).collect()
        } else {
            options.stopwords.iter().cloned().collect()
        };

        let words: Vec<&str> = cleaned
            .split(DEFAULT_SEPARATOR)
            .filter(|w| !stopwords_set.contains(*w))
            .collect();
        cleaned = words.join(DEFAULT_SEPARATOR);
    }

    // 14. Final user-specific replacements
    for (old_val, new_val) in &options.replacements {
        cleaned = cleaned.replace(old_val, new_val);
    }

    // 15. Smart truncate
    if options.max_length > 0 {
        cleaned = smart_truncate(
            &cleaned,
            options.max_length,
            options.word_boundary,
            DEFAULT_SEPARATOR,
            options.save_order,
        );
    }

    // 16. Custom separator
    if options.separator != DEFAULT_SEPARATOR {
        cleaned = cleaned.replace(DEFAULT_SEPARATOR, options.separator);
    }

    cleaned
}
