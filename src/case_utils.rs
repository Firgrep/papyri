use once_cell::sync::Lazy;
use std::collections::HashSet;
extern crate inflector;
use inflector::Inflector;

pub fn qualified_capitalize(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut new_string = String::new();

    for (index, word) in words.iter().enumerate() {
        if index != 0 {
            new_string.push(' ');
        }
        if index == 0 {
            let capitalized_word = word.to_title_case();
            new_string.push_str(&capitalized_word);
        } else if NON_CAPITALIZE_WORDS.contains(word) {
            new_string.push_str(word);
        } else {
            let capitalized_word = word.to_title_case();
            new_string.push_str(&capitalized_word);
        }
    }

    return new_string;
}

static NON_CAPITALIZE_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: Vec<&str> = vec![
        "of",
        "and",
        "von",
        "the",
        "a",
        "an",
        "in",
        "on",
        "at",
        "by",
        "for",
        "with",
        "to",
        "from",
        "into",
        "onto",
        "upon",
        "among",
        "between",
        "within",
        "without",
        "during",
        "through",
        "throughout",
        "across",
        "but",
        "or",
        "nor",
        "yet",
        "so",
    ];
    words.into_iter().collect()
});
