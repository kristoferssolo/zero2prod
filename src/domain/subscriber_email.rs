use std::str::FromStr;

use validator::validate_email;

#[derive(Debug, Clone)]
pub struct SubscriberEmail(String);

impl TryFrom<String> for SubscriberEmail {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !validate_email(&value) {
            return Err(format!("{} is not a valid subscriber email.", value));
        }
        Ok(Self(value))
    }
}

impl FromStr for SubscriberEmail {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use quickcheck::Arbitrary;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        assert_err!(SubscriberEmail::from_str(""));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        assert_err!(SubscriberEmail::from_str("ursuladomain.com"));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        assert_err!(SubscriberEmail::from_str("@domain.com"));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::try_from(valid_email.0).is_ok()
    }
}
