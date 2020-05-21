//! Implements the SentenceToken module
//!
//! The main struct in this module is the [`SentenceToken`], a
//! class to store sentence objects. In general a sentente
//! will contain a String, and methods to extract the
//! tokens in that String and put them into a vector or [`WordTokens`]
//!
//! [`SentenceToken`]: struct.SentenceToken.html
//! [`WordToken`]: struct.WordToken.html

pub use crate::word_token::WordToken;

#[derive(Debug)]
pub struct SentenceToken {
    pub sentence: String,
    pub v_words: Vec<WordToken>,
}

impl SentenceToken {
    /// from method
    ///
    /// # Arguments
    ///
    /// * text - a string reference
    ///
    /// # Return
    ///
    /// * a SentenceToken
    ///
    pub fn from(text: &str) -> SentenceToken {
        SentenceToken {
            sentence: String::from(text),
            v_words: Vec::<WordToken>::new(),
        }
    }

    /// len
    ///
    /// Returns the length in characters of the sentenc.e
    ///
    /// # Return
    ///
    /// * usize - number of characters in the sentence.
    pub fn len(&self) -> usize {
        self.sentence.len()
    }

    pub fn next_word(&mut self, idxo: usize) -> usize {
        let bytes = self.sentence[idxo..].as_bytes();
        let mut startidx: usize = 0;

        for (idx, &items) in bytes.iter().enumerate() {
            if items == b' ' {
                if idx == 0 {
                    startidx = startidx + 1;
                    continue;
                }

                //return (&self.sentence[idxo..idxo+idx], idxo+idx);
                let start = idxo + startidx;
                let end = idxo + idx;
                let word = &self.sentence[start..end];
                self.v_words.push(WordToken::from(word, start, end));
                return idxo + idx;
            }
        }
        let start = idxo + startidx;
        let end = self.len();
        let word = &self.sentence[start..end];
        self.v_words.push(WordToken::from(word, start, end));
        return self.len();
    }
}
