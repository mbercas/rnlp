#[cfg(test)]
mod sentence_token_tests {
    use nlp::*;

    #[test]
    fn from_with_normal_string() {
        let sample = "This is a normal sentence";
        let dut = SentenceToken::from(sample);

        assert_eq!(sample, dut.sentence.to_string());
        assert_eq!(sample.len(), dut.len());
        assert_eq!(5, dut.number_of_tokens());
    }

    #[test]
    fn from_with_empty_string() {
        let sample = "";
        let dut = SentenceToken::from(sample);

        assert_eq!(sample, dut.sentence.to_string());
        assert_eq!(sample.len(), dut.len());
        assert_eq!(0, dut.number_of_tokens());
    }

    #[test]
    fn tokenize_with_many_spaces() {
        let sample = "  this is a   string with     many  spaces    ";
        let dut = SentenceToken::from(sample);

        assert_eq!(sample, dut.sentence.to_string());
        assert_eq!(sample.len(), dut.len());
        assert_eq!(7, dut.number_of_tokens());

    }
    

    #[test]
    fn tokenize_with_many_spaces_and_punctuation_marks() {
        let sample = "  this is a   string; with     many  spaces  and symbols!  ";
        let dut = SentenceToken::from(sample);

        assert_eq!(sample, dut.sentence.to_string());
        assert_eq!(sample.len(), dut.len());
        for w in &dut.v_words {
            println!("{:?}", w);
        }
        assert_eq!(11, dut.number_of_tokens());

    }

    #[test]
    fn tokenize_just_punctuaation_marks() {
        let sample = "!";
        let dut = SentenceToken::from(sample);

        assert_eq!(sample, dut.sentence.to_string());
        assert_eq!(sample.len(), dut.len());
        for w in &dut.v_words {
            println!("{:?}", w);
        }
        assert_eq!(1, dut.number_of_tokens());

    }

    
}
