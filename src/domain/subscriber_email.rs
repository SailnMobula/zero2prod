use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<SubscriberEmail, String> {
        if !validate_email(&email) {
            return Err(format!("{} is not a valid email", &email));
        }
        Ok(Self(email))
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use claim::{assert_err, assert_ok};

    use crate::domain::SubscriberEmail;

    #[test]
    fn parse_empty_email_is_invalid() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn parse_whitespace_email_is_invalid() {
        let email = "     ".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn parse_missing_at_email_is_invalid() {
        let email = "ursula.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn parse_missing_subject_email_is_invalid() {
        let email = "@gmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn parse_nice_email_is_valid() {
        let email = "ursual@gmail.com".to_string();
        assert_ok!(SubscriberEmail::parse(email));
    }
}
