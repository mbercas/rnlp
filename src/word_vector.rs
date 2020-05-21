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
/// WordVector struct
///
/// # Attributes
///
/// * words - a HashMap of key:String and count of appearances of the string
///
/// *NOTE* all Strings are coverted to lower case before storing them.
pub struct WordVector {
    pub words: HashMap<String, u32>,
}

impl WordVector {
    /// create an empry WordVector
    ///
    pub fn new() -> WordVector {
        WordVector {
            words: HashMap::new(),
        }
    }

    /// Add a word to the WordVector
    ///
    /// Appends the word passed as argument to the word vector,
    /// incrementing by one the times the word appears if the
    /// word already was contained in the Wordvector
    ///
    /// # Arguments
    /// * `word` - &str containing the word
    ///
    /// *NOTE* capitalized words are converter to lower case before
    ///        adding them to the WordVector
    ///
    pub fn add_word(&mut self, word: &str) {
        let count = self.words.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    /// Adds the contents of a second word vector into this one
    ///
    /// # Arguments
    ///
    /// * `other` - a &WordVector containing the words to be added to the
    ///             current word vector
    pub fn add_wordvector(&mut self, other: &WordVector) {
        for (word, other_count) in &other.words {
            let new_count = self.words.entry(word.to_string()).or_insert(*other_count);
            *new_count += other_count;
        }
    }
}
