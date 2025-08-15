use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Matched {
    Yes,
    Partially,
    No,
}

struct MatchedVisitor;

impl Visitor<'_> for MatchedVisitor {
    type Value = Matched;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("+, ? or -")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "+" => Ok(Self::Value::Yes),
            "?" => Ok(Self::Value::Partially),
            "-" => Ok(Self::Value::No),
            _ => Err(E::custom(format!("must be one of +, ? or -: {v}"))),
        }
    }
}

impl Display for Matched {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Yes => "+",
                Self::Partially => "?",
                Self::No => "-",
            }
        )
    }
}

impl From<&str> for Matched {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Yes,
            "?" => Self::Partially,
            "-" => Self::No,
            _ => Self::No,
        }
    }
}

impl From<String> for Matched {
    fn from(value: String) -> Self {
        Self::from(&value[..])
    }
}

impl Serialize for Matched {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Matched {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MatchedVisitor)
    }
}

#[test]
fn test_serde_matched() {
    use serde_test::{Token, assert_tokens};

    let matched = Matched::Yes;
    assert_tokens(&matched, &[Token::Str("+")]);
}
