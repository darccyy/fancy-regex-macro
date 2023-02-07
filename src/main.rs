use fancy_regex_macro::regex;

fn main() {
    let number = regex!("[0-9]+");

    assert!(number.is_match("1824").unwrap());
    assert!(!number.is_match("this wont match").unwrap());
}
