use crate::{Matches, PUZZLE_LETTERS_COUNT, PuzzleSolution, SubmitLetter};

use std::{collections::HashMap, fmt::Display};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmitWord<const N: usize = PUZZLE_LETTERS_COUNT>(pub [SubmitLetter; N]);

impl<const N: usize> SubmitWord<N> {
    pub const SEPARATOR: &str = ",";

    pub fn new(letters: [SubmitLetter; N]) -> Self {
        Self(letters)
    }

    pub fn tint(answer: &PuzzleSolution<N>, solution: &PuzzleSolution<N>) -> Self {
        // Tint matches letters
        let mut unparsed_map = solution.inner().iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });
        let letters: Vec<(char, Option<Matches>)> = answer
            .inner()
            .iter()
            .enumerate()
            .map(|(index, &c)| {
                (
                    c,
                    if solution.inner()[index] == c {
                        unparsed_map.entry(c).and_modify(|count| *count -= 1);
                        Some(Matches::Yes)
                    } else {
                        None
                    },
                )
            })
            .collect();

        // Tint partially matches and unmatched letters
        let letters: Vec<SubmitLetter> = letters
            .iter()
            .map(|&(c, matches)| match matches {
                Some(matches) => SubmitLetter::new(c, matches),
                None => {
                    if unparsed_map
                        .get(&c)
                        .map(|&count| count > 0)
                        .unwrap_or(false)
                    {
                        unparsed_map.entry(c).and_modify(|count| *count -= 1);
                        SubmitLetter::new(c, Matches::Partially)
                    } else {
                        SubmitLetter::new(c, Matches::No)
                    }
                }
            })
            .collect();

        Self(letters[..].try_into().unwrap())
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn to_vec(&self) -> Vec<&SubmitLetter> {
        self.0.iter().collect()
    }

    pub fn into_vec(self) -> Vec<SubmitLetter> {
        self.0.into_iter().collect()
    }
}

impl<const N: usize> Display for SubmitWord<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.map(|l| l.to_string()).join(Self::SEPARATOR))
    }
}

impl<const N: usize> Serialize for SubmitWord<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(Some(self.0.len()))?;
        for l in self.0 {
            s.serialize_element(&l)?;
        }
        s.end()
    }
}

impl<'de, const N: usize> Deserialize<'de> for SubmitWord<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SubmitWordVisitor::<N>)
    }
}

struct SubmitWordVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for SubmitWordVisitor<N> {
    type Value = SubmitWord<N>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format!("a word of length {N}"))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut letters: Vec<SubmitLetter> = Vec::new();
        while let Ok(Some(letter)) = seq.next_element() {
            letters.push(letter);
        }

        match &letters[..].try_into() {
            Ok(letters) => Ok(Self::Value::new(*letters)),
            Err(err) => Err(serde::de::Error::custom(format!(
                "value must contain exactly {}: {letters:?}, {err}",
                match N {
                    1 => "1 element",
                    n => &format!("{n} elements"),
                }
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matches, SubmitLetter, SubmitWord};

    use serde_test::{Token, assert_tokens};

    #[test]
    fn test_serde() {
        let word = SubmitWord([
            SubmitLetter::new('R', Matches::Yes),
            SubmitLetter::new('U', Matches::No),
            SubmitLetter::new('S', Matches::Partially),
            SubmitLetter::new('T', Matches::Yes),
            SubmitLetter::new('Y', Matches::Partially),
        ]);

        fn get_letter_tokens(letter: &SubmitLetter) -> Vec<Token> {
            vec![
                Token::Struct {
                    name: stringify!(SubmitLetter),
                    len: 2,
                },
                Token::Str("letter"),
                Token::Char(letter.letter),
                Token::Str("matches"),
                Token::UnitVariant {
                    name: stringify!(Matches),
                    variant: letter.matches.to_str(),
                },
                Token::StructEnd,
            ]
        }

        let tokens: Vec<Token> = word
            .to_vec()
            .into_iter()
            .flat_map(get_letter_tokens)
            .collect();

        assert_tokens(
            &word,
            &[
                &[Token::Seq {
                    len: Some(word.len()),
                }],
                &tokens[..],
                &[Token::SeqEnd],
            ]
            .concat(),
        );
    }
}
