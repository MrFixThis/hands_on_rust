use anyhow::{Error, Ok, Result};
use serde::Serialize;

use crate::handler::Source;

/// A match result from a Boyer-Moore search.
#[derive(Serialize, Default)]
pub struct Match {
    ocurrences: usize,
}

/// The implementation of the *Bad Character Heuristic* approach of the
/// `Boyer-Moore Pattern Searching Algorithm.`
#[derive(Debug)]
pub struct BoyerMooreSearcher {
    text: Vec<u8>,
    pattern: Vec<u8>,
}

impl BoyerMooreSearcher {
    /// The number of characters of the extended `ASCII` table.
    ///
    /// This value will be used to create a [`Vec`] with this size
    /// to store the ocurrences of the `pattern`'s characters.
    const NUM_CHARS: u8 = 255;

    /// Creates a new [`BoyerMooreSearcher`] instance over an specified [`Source`].
    pub fn new(src: Source) -> Self {
        Self {
            text: src.text.into_bytes(),
            pattern: src.pattern.into_bytes(),
        }
    }

    /// Performs a `pattern` search against the specified text to match.
    ///
    /// Returns the number of `ocurrences` found in the text and, if this number
    /// is greater than 0, the source text with the `highlighted` matches.
    pub fn search(self) -> Result<Match> {
        let txt_len = self.text.len();
        let pat_len = self.pattern.len();

        // if the length of the pattern is greater than the text length
        if txt_len < pat_len {
            return Err(Error::msg("pattern greater than file's content"));
        }

        let bad_char = Self::preprocess_pattern(&self.pattern);
        let mut shift: usize = 0;
        let mut ocurrences: usize = 0;
        while shift <= (txt_len - pat_len) {
            let mut j = (pat_len - 1) as isize;

            // keep decrementing j while pattern matches against the text at this
            // shift
            while j as usize <= pat_len && self.pattern[j as usize] == self.text[shift + j as usize]
            {
                j -= 1;
            }

            // If the pattern is present at current shift, then index j will
            // become -1 after the above loop
            if j < 0 {
                // Shift the pattern so that the next character in text aligns
                // with the last occurrence of it in pattern
                shift += if shift + pat_len < txt_len {
                    pat_len.wrapping_sub(bad_char[self.text[shift + pat_len] as usize] as usize)
                } else {
                    1
                };
                // incrementing the number of ocurrences of the pattern in the text
                ocurrences += 1;
            } else {
                // Shift the pattern so that the bad character in text aligns
                // with the last occurrence of it in pattern. The max function
                // is used to make sure that we get a positive shift
                shift += (1.max(j - bad_char[self.text[shift + j as usize] as usize])) as usize;
            }
        }

        Ok(Match { ocurrences })
    }

    /// Preprocesses the `pattern` being seached and stablish a [`Vec`]
    /// filled with the `pattern`'s characters coincidences in the searched `text`.
    fn preprocess_pattern(pattern: &[u8]) -> Vec<isize> {
        // initially, there is not last ocurrence of the bad character in the
        // pattern, so -1 is assigned
        let mut bad_char = vec![-1isize; Self::NUM_CHARS as usize];

        // correcting the vector, so the characters of the pattern get aligned
        pattern
            .iter()
            .enumerate()
            .for_each(|(i, &c)| bad_char[c as usize] = i as isize);
        bad_char
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn pattern_get_matched_against_text() {
        let text = fs::read_to_string("testdata/lorem_ipsum.txt").unwrap();
        let source = Source {
            text,
            pattern: "dolore".to_owned(),
        };

        let searcher = BoyerMooreSearcher::new(source);
        assert!(searcher.search().unwrap().ocurrences > 0)
    }

    #[test]
    #[should_panic]
    fn pattern_is_greater_than_text() {
        let source = Source {
            text: r#""Nothing better than Rust." - MrFixThis, 2023"#.to_owned(),
            pattern: r#""Nothing better than C." - Linus Torvalds, idk when"#.to_owned(),
        };

        let searcher = BoyerMooreSearcher::new(source);
        searcher
            .search()
            .expect("pattern's length should be less than text's length");
    }
}
