#![forbid(unsafe_code)]

pub type ReplacementPair = (String, String);

fn add_uppercase(mut pairs: Vec<(&str, &str)>) -> Vec<ReplacementPair> {
    let mut result: Vec<ReplacementPair> = Vec::new();
    for (ch, xlate) in pairs.drain(..) {
        let upper_ch = ch.to_uppercase();
        let mut cap_xlate = String::new();
        let mut chars = xlate.chars();
        if let Some(first) = chars.next() {
            cap_xlate.push_str(&first.to_uppercase().to_string());
            cap_xlate.push_str(chars.as_str());
        }

        if ch != upper_ch && !result.iter().any(|(c, _)| c == &upper_ch) {
            result.insert(0, (upper_ch, cap_xlate));
        }
        if !result.iter().any(|(c, _)| c == ch) {
            result.push((ch.to_string(), xlate.to_string()));
        }
    }
    result
}

pub fn get_cyrillic() -> Vec<ReplacementPair> {
    add_uppercase(vec![
        ("ё", "e"),
        ("я", "ya"),
        ("х", "h"),
        ("у", "y"),
        ("щ", "sch"),
        ("ю", "u"),
    ])
}

pub fn get_german() -> Vec<ReplacementPair> {
    add_uppercase(vec![
        ("ä", "ae"),
        ("ö", "oe"),
        ("ü", "ue"),
    ])
}

pub fn get_greek() -> Vec<ReplacementPair> {
    add_uppercase(vec![
        ("χ", "ch"),
        ("Ξ", "X"),
        ("ϒ", "Y"),
        ("υ", "y"),
        ("ύ", "y"),
        ("ϋ", "y"),
        ("ΰ", "y"),
    ])
}

pub fn get_pre_translations() -> Vec<ReplacementPair> {
    let mut all = get_cyrillic();
    all.extend(get_german());
    all.extend(get_greek());
    all
}
