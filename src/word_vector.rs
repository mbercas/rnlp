//! Implements a WordVector module
//!
//! This module contains the [`WordVector`] struct.
//!
//! The WordVector maintains a dictionary of words and the counts
//! of the times the word has been added.
//!
//! [`WordVector`]: struct.WordVector.html



use std::collections::HashMap;


#[derive(Debug)]
pub struct WordVector {
    pub words: HashMap<String, u32>,        
}

impl WordVector {

    pub fn new() -> WordVector {
        WordVector {
            words: HashMap::new()
        }  
    }
    
    pub fn add_word(&mut self, word: &str) {
        let count = self.words.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    /// Adds the contents of a second word vector into this one
    ///
    pub fn add_wordvector(&mut self, other: &WordVector) {
        for (word, other_count) in &other.words {
            let new_count = self.words.entry(word.to_string()).or_insert(*other_count);
            *new_count += other_count;
        }
    }
}
