#[cfg(test)]
mod word_vector_tests {
    use nlp::*;

    #[test]
    fn add_non_existing_word() {
        let mut dut = WordVector::new();

        let new_word = String::from("calexico");

        dut.add_word(new_word.as_str());

        assert_eq!(1, dut.words.len());
        assert_eq!(1, dut.words["calexico"]);
    }

    #[test]
    fn add_existing_word() {
        let mut dut = WordVector::new();

        let uncap_new_word = String::from("calexico");
        let cap_new_word = String::from("Calexico");

        dut.add_word(uncap_new_word.as_str());

        assert_eq!(1, dut.words.len());
        assert_eq!(1, dut.words["calexico"]);

        dut.add_word(cap_new_word.as_str());

        assert_eq!(1, dut.words.len());
        assert_eq!(2, dut.words["calexico"]);

        dut.add_word(uncap_new_word.as_str());

        assert_eq!(1, dut.words.len());
        assert_eq!(3, dut.words["calexico"]);
    }

    #[test]
    fn add_different_words() {
        let mut dut = WordVector::new();

        let passes = vec![1, 2, 3];
        let words = vec!["uno", "two", "drei", "cuatre", "5", ",", "Two"];
        for pass in passes {
            for w in &words {
                println!("{} - {}", pass, w);
                dut.add_word(w);
                match dut.words.get(&w.to_lowercase().to_string()) {
                    Some(_count) => continue,
                    None => assert!(false, format!(" {}: key not found", w)),
                }
            }
            assert_eq!(words.len() - 1, dut.words.len());
        }
        match dut.words.get("uno") {
            Some(_count) => assert_eq!(3, *_count),
            None => assert!(false, "uno: key not found"),
        }
        match dut.words.get("two") {
            Some(_count) => assert_eq!(6, *_count),
            None => assert!(false, "two: key not found"),
        }
    }

    #[test]
    fn test_add_two_word_vectors_add_empty_wordvector() {
        let mut dut_first = WordVector::new();
        let dut_second = WordVector::new();

        let words = vec!["uno", "two", "drei", "cuatre", "5", ",", "Two"];
        for w in &words {
            dut_first.add_word(w);
        }

        dut_first.add_wordvector(&dut_second);

        match dut_first.words.get("uno") {
            Some(_count) => assert_eq!(1, *_count),
            None => assert!(false, "uno: key not found"),
        }
        match dut_first.words.get("two") {
            Some(_count) => assert_eq!(2, *_count),
            None => assert!(false, "two: key not found"),
        }
    }

    #[test]
    fn test_add_two_word_vectors_add_same_wordvector() {
        let mut dut_first = WordVector::new();
        let mut dut_second = WordVector::new();

        let words = vec!["uno", "two", "drei", "cuatre", "5", ",", "Two"];
        for w in &words {
            dut_first.add_word(w);
            dut_second.add_word(w);
        }

        for _i in 1..3 {
            dut_first.add_wordvector(&dut_second);
        }

        match dut_first.words.get("uno") {
            Some(_count) => assert_eq!(3, *_count),
            None => assert!(false, "uno: key not found"),
        }
        match dut_first.words.get("two") {
            Some(_count) => assert_eq!(6, *_count),
            None => assert!(false, "two: key not found"),
        }
    }
}
