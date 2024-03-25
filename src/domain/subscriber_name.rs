use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl TryFrom<String> for SubscriberName {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let is_empty_or_whitespace = value.trim().is_empty();
        let is_too_long = value.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters =
            value.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            return Err(format!("{} is not a valid subscriber name.", value));
        }
        Ok(Self(value))
    }
}

impl FromStr for SubscriberName {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use claims::{assert_err, assert_ok};

    use super::*;
    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        assert_ok!(SubscriberName::try_from("ē".repeat(256)));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        assert_err!(SubscriberName::try_from("a".repeat(257)));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        assert_err!(SubscriberName::from_str(" "));
    }

    #[test]
    fn empty_string_is_rejected() {
        assert_err!(SubscriberName::from_str(""));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for ch in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            assert_err!(SubscriberName::try_from(ch.to_string()));
        }
    }

    #[test]
    fn a_valid_name_is_try_fromd_successfully() {
        assert_ok!(SubscriberName::from_str("Ursula Le Guin"));
    }
}
