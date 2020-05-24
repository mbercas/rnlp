//! Common structures for NLP package


use std::collections::HashMap;

pub struct PunctuationMarks {
    pub symbols: HashMap<String, String>,
}

impl PunctuationMarks {
    pub fn new() -> PunctuationMarks {
        let mut pmarks = PunctuationMarks {
            symbols: HashMap::new(),
        };
            
        let  _symbols =  [(".", "dot"),
                          (",", "comma"),
                          (":", "colon"),
                          (";", "semi-colon"),
                          ("'", "single quote"),
                          ("\"", "double quotes"),
                          ("\\", "forward-slash"),
                          ("/", "backward-slash"),
                          ("?", "question-mark"),
                          ("!", "exclamation-mark"),
                          ("(", "open-c-bracket"),
                          (")", "close-c-bracket")];

        for (key, value) in &_symbols {
            pmarks.symbols.insert(String::from(*key), String::from(*value));
        }

        pmarks
    }
}

/// Checks if the string slice passed as an argument is a
/// symbol in the PunctuationMarks list
///
/// # Arguments
///
///  * symbol: string slice with the symbol to be checked
///  * punc_marks; a struct holding the punctuation markds
///
/// * Returns
///
/// A bool, set to true if the input is a punctuation mark
///
pub fn is_punctuation_mark(symbol: &str, punc_marks: &PunctuationMarks) -> bool {
    punc_marks.symbols.contains_key(symbol)
}


