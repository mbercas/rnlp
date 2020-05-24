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
pub use crate::common::is_punctuation_mark;
pub use crate::common::PunctuationMarks;


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
        let mut st = SentenceToken {
            sentence: String::from(text),
            v_words: Vec::<WordToken>::new(),
        };

        st.tokenize();
        st
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

    /// number_of_tokens
    ///
    /// Returns the number of tokens in the sentence
    ///
    /// # Return
    ///  * usize -  number of tokens in the sentence
    ///
    pub fn number_of_tokens(&self) -> usize {
        self.v_words.len()
    }

    fn tokenize(&mut self) {

        let mut idx = 0;
        loop {
            idx = self.next_word(idx);

            if idx == self.len() {
                break;
            }

        }
    }
    
    fn next_word(&mut self, idxo: usize) -> usize {
        let bytes = self.sentence[idxo..].as_bytes();
        let mut startidx: usize = 0;
        let punc_marks = PunctuationMarks::new(); // Todo; there must be a better way to do this
                                                  // than initilizing here

        for (idx, &items) in bytes.iter().enumerate() {
            // remove spaces at the begginging of the string
            if items == b' ' {
                if idx == 0 {
                    startidx = startidx + 1;
                    continue;
                } 

                //return (&self.sentence[idxo..idxo+idx], idxo+idx);
                let start = idxo + startidx;
                let end = idxo + idx;
                let word = &self.sentence[start..end];
                
                if start != end {
                    self.v_words.push(WordToken::from(word, start, end));
                }
                
                return end;

            } else if is_punctuation_mark(&self.sentence[idxo+idx..idxo+idx+1], &punc_marks) {
                let start = idxo + startidx;
                // check if the substring is just a punc. mark or the mark appeared a the end
                // of a string.
                let end = if idx == 0 { idxo + idx + 1 } else { idxo + idx }; 
                
                let word = &self.sentence[start..end];
                
                self.v_words.push(WordToken::from(word, start, end));
                
                return end;                    
            }
        }
        let start = idxo + startidx; 
        let end = self.len();
        let word = &self.sentence[start..end];

        if start != end {
            self.v_words.push(WordToken::from(word, start, end));
        }

        return self.len();
    }
}
