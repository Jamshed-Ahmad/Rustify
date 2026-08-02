#![forbid(unsafe_code)]

use clap::Parser;
use slugify::{slugify, SlugifyOptions, DEFAULT_SEPARATOR};
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(name = "slugify", author = "Senior Systems Engineer", version = "8.0.4", about = "Slug string")]
struct Cli {
    /// Text to slugify
    #[arg(trailing_var_arg = true)]
    input_string: Vec<String>,

    /// Take text from STDIN
    #[arg(long)]
    stdin: bool,

    /// Do not convert HTML entities to unicode
    #[arg(long = "no-entities", action = clap::ArgAction::SetFalse, default_value_t = true)]
    entities: bool,

    /// Do not convert HTML decimal to unicode
    #[arg(long = "no-decimal", action = clap::ArgAction::SetFalse, default_value_t = true)]
    decimal: bool,

    /// Do not convert HTML hexadecimal to unicode
    #[arg(long = "no-hexadecimal", action = clap::ArgAction::SetFalse, default_value_t = true)]
    hexadecimal: bool,

    /// Output string length, 0 for no limit
    #[arg(long, default_value_t = 0)]
    max_length: usize,

    /// Truncate to complete word even if length ends up shorter than --max-length
    #[arg(long)]
    word_boundary: bool,

    /// When set and --max-length > 0 return whole words in initial order
    #[arg(long)]
    save_order: bool,

    /// Separator between words. Default '-'
    #[arg(long, default_value = DEFAULT_SEPARATOR)]
    separator: String,

    /// Words to discount
    #[arg(long, num_args = 1..)]
    stopwords: Vec<String>,

    /// Python regex pattern for disallowed characters
    #[arg(long)]
    regex_pattern: Option<String>,

    /// Activate case sensitivity
    #[arg(long = "no-lowercase", action = clap::ArgAction::SetFalse, default_value_t = true)]
    lowercase: bool,

    /// Replacement rules of form ORIGINAL->REPLACED
    #[arg(long, num_args = 1..)]
    replacements: Vec<String>,

    /// Allow unicode characters
    #[arg(long)]
    allow_unicode: bool,
}

fn main() {
    let cli = Cli::parse();

    let text_input = if cli.stdin {
        let mut buffer = String::new();
        let _ = io::stdin().read_to_string(&mut buffer);
        buffer
    } else {
        cli.input_string.join(" ")
    };

    let parsed_replacements: Vec<(String, String)> = cli
        .replacements
        .iter()
        .filter_map(|r| {
            let mut parts = r.splitn(2, "->");
            let k = parts.next()?.to_string();
            let v = parts.next()?.to_string();
            Some((k, v))
        })
        .collect();

    let options = SlugifyOptions {
        entities: cli.entities,
        decimal: cli.decimal,
        hexadecimal: cli.hexadecimal,
        max_length: cli.max_length,
        word_boundary: cli.word_boundary,
        separator: &cli.separator,
        save_order: cli.save_order,
        stopwords: cli.stopwords,
        regex_pattern: cli.regex_pattern,
        lowercase: cli.lowercase,
        replacements: parsed_replacements,
        allow_unicode: cli.allow_unicode,
    };

    let result = slugify(&text_input, &options);
    println!("{}", result);
}
