//! WordToken module
//!
//! The main struct in the [`WordToken`] which is used to record
//! word strings as well as their location in the sentence.
//!
//! Typically [`WordToken`] is used with [`SentenceToken`],
//! the sentence tokenizer will split a sentence into the words
//! and will record the locations of the words.
//!
//! NOTE: words are always stored in lower-case
//! [`WordToken`]: struct.WordToken.html
//! [`SentenceToken`]: struct.SentenceToken.html

#[derive(Debug)]
pub struct WordToken {
    pub start: usize,
    pub end: usize,
    pub word: String,
}

impl WordToken {
    pub fn from(word: &str, start: usize, end: usize) -> WordToken{
        WordToken {
            word: String::from(word.to_lowercase()),
            start,
            end,
        }
    }
}

