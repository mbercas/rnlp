#[cfg(test)]
mod sentence_token_tests {
    use nlp::*;

    #[test]
    fn from_with_normal_string() {
        let sample = "This is a normal sentence";
        let dut = SentenceToken::from(sample);

        assert_eq!(sample, dut.sentence.to_string());
        assert_eq!(sample.len(), dut.len());
    }

    #[test]
    fn from_with_empty_string() {
        let sample = "";
        let dut = SentenceToken::from(sample);

        assert_eq!(sample, dut.sentence.to_string());
        assert_eq!(sample.len(), dut.len());
    }
}
