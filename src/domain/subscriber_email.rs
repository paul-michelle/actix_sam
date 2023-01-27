use validator::validate_email;

pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn from_string(s: String) -> Result<Self, String> {
        if !validate_email(&s) {
            return Err("Invalid email".to_string());
        }
        Ok(SubscriberEmail(s))
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subscriber_email_ref() {
        let email_literal = "rob.pike@gmail.com";
        let email = SubscriberEmail::from_string(email_literal.to_string()).unwrap();
        assert_eq!(email_literal, email.as_ref())
    }
}
