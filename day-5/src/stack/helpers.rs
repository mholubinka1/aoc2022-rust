use lazy_static::lazy_static;
use regex::Regex;

pub fn str_strip_numbers(s: &str) -> Vec<usize> {
    lazy_static! {
        static ref FIND_NUMBERS: Regex = Regex::new(r"\d+").unwrap();
    }
    FIND_NUMBERS.find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}