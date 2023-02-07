#[doc(hidden)]
pub use fancy_regex::Regex;
#[doc(hidden)]
pub type Lazy = once_cell::sync::Lazy<Regex>;

#[macro_export]
macro_rules! regex {
    ($re:expr) => {{
        static RE: $crate::Lazy =
            $crate::Lazy::new(|| $crate::Regex::new($re).expect("Failed to parse static regex"));
        $crate::Lazy::force(&RE)
    }};
}

#[cfg(test)]
mod tests {
    use super::regex;

    #[test]
    fn match_numbers() {
        let number = regex!("[0-9]+");
        assert!(number.is_match("1824").unwrap());
        assert!(!number.is_match("this wont match").unwrap());
    }

    #[test]
    #[should_panic]
    fn invalid_syntax() {
        let _ = regex!("?[");
    }
}
