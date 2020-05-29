#[cfg(test)]

/// Unit tests for the WordToken module
mod word_token_test {

    use nlp::*;

    #[test]
    fn word_token_from_normal_word_test() {
        let word = String::from("word");
        let sz_word = word.len();

        let dut = WordToken::from(&word, 0, sz_word).unwrap();

        assert_eq!(word, dut.word);
    }

    #[test]
    fn word_token_from_capitalized_word_test() {
        let word = String::from("WoRd");
        let sz_word = word.len();

        let dut = WordToken::from(&word, 0, sz_word).unwrap();

        assert_eq!("word", dut.word);
    }

    #[test]
    fn word_token_invalid_indexes() {
        let word = String::from("word");
        let sz_word = word.len();

        assert_eq!(true, WordToken::from(&word, 0, sz_word + 1).is_err());
    }
}
