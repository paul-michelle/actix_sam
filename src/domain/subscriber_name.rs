use unicode_segmentation::UnicodeSegmentation;

const NAME_MAX_LENGTH: u8 = 255;
const FORBIDDEN_CHARS: [char; 9] = ['{', '}', '<', '>', '"', '/', '\\', '(', ')'];
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn from_string(s: String) -> Result<Self, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > NAME_MAX_LENGTH as usize;
        let contains_forbidden_chars = s.chars().any(|char| FORBIDDEN_CHARS.contains(&char));

        match is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            true => Err("Invalid name".to_string()),
            false => Ok(Self(s)),
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
