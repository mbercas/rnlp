
#[cfg(test)]
mod common_tests {
    use nlp::*;

    #[test]
    fn check_letter_is_not_punctuation() {
        let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";

        for i in 0..lowercase_letters.len() {
            assert_eq!(false,
                       is_punctuation_mark(
                           &lowercase_letters[i..i+1],
                           &PunctuationMarks::new()));
        }

        let uppercase_letters =  &lowercase_letters.to_uppercase();
        for i in 0..uppercase_letters.len() {
            assert_eq!(false,
                       is_punctuation_mark(
                           &uppercase_letters[i..i+1],
                           &PunctuationMarks::new()));
        }
        
    }

        
}
