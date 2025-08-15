use crate::Matched;

use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmitLetter(char, Matched);

struct SubmitLetterVisitor;

impl Visitor<'_> for SubmitLetterVisitor {
    type Value = SubmitLetter;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a string formatted like {char}{+/?/-}")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut chars = v.chars();
        if v.len() == 2 {
            let c = chars.next().unwrap().to_ascii_uppercase();
            if !c.is_ascii_alphabetic() {
                return Err(E::custom(format!(
                    "the character pattern must be ascii alphabetic: {}",
                    c
                )));
            }

            let matched = Matched::from(chars.next().unwrap().to_string());
            Ok(Self::Value::new(c, matched))
        } else {
            Err(E::custom(format!(
                "value must contain exactly 2 characters: {}",
                v
            )))
        }
    }
}

impl SubmitLetter {
    pub fn new(c: char, matched: Matched) -> Self {
        Self(c.to_ascii_uppercase(), matched)
    }
}

impl Display for SubmitLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Serialize for SubmitLetter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SubmitLetter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SubmitLetterVisitor)
    }
}

#[test]
fn test_serde_submit_letter() {
    use serde_test::{Token, assert_tokens};

    let submit_letter = SubmitLetter::new('A', Matched::Partially);
    assert_tokens(&submit_letter, &[Token::Str("A?")]);
}
