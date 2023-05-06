use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<Self, String> {
        let is_empty = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let has_forbidden_chars = name.chars().any(|s| forbidden_chars.contains(&s));

        if is_empty || is_too_long || has_forbidden_chars {
            return Err("Name is not valid".to_string());
        }

        Ok(SubscriberName(name))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};

    use crate::domain::SubscriberName;

    #[test]
    fn a_grapheme_256_is_valid() {
        let name = "ё".repeat(256).to_string();
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_grapheme_257_is_invalid() {
        let name = "ё".repeat(257).to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn parse_with_whitespace_only_is_invalid() {
        let name = "   ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn parse_with_empty_name_is_invalid() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name.to_string()));
    }

    #[test]
    fn parse_with_forbidden_chars_is_invalid() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn parse_with_cool_name_is_valid() {
        let name = "Usrula K Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
