// Implements the SentenceToken module

pub use crate::word_token::WordToken; 

#[derive(Debug)]
pub struct SentenceToken {
    pub sentence: String,
    pub v_words: Vec<WordToken>,
}


impl SentenceToken {

    pub fn from(text: &str ) -> SentenceToken {
        SentenceToken {
            sentence: String::from(text),
            v_words: Vec::<WordToken>::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.sentence.len()
    }
    
    pub fn next_word(&mut self, idxo: usize) -> usize {
        let bytes = self.sentence[idxo..].as_bytes();
        let mut startidx :usize = 0;
        
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
                self.v_words.push(
                    WordToken::from(word, start, end)
                );
                return idxo + idx;   
            }
        }
        let start = idxo + startidx;
        let end = self.len();
        let word = &self.sentence[start..end];
        self.v_words.push(
            WordToken::from(word, start, end)
        );
        return self.len();
    }
}
