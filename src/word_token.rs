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

/// Returns a word token containing the string passed
/// as reference and the indexes whithing the sentence
///
/// # Arguments
///
///  * word - reference to a string containing the word.
///  * start - index of the first letter of word in the sentence
///  * end - index of the last lettre of word in the sentence
///
/// # Returns Result<WordToken>
///
/// If the distance between the indexes does not match the
/// length of the word returns
///
impl WordToken {
    pub fn from(word: &str, start: usize, end: usize) -> Result<WordToken, &str> {
        if end - start != word.len() {
            Err("Length of word does not match distance between indexes")
        } else {
            Ok(WordToken {
                word: String::from(word.to_lowercase()),
                start,
                end,
            })
        }
    }
}
