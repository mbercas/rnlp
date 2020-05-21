// Analize English text and extracts features

use nlp::*;


fn main() {
    let mut sentence = SentenceToken::from("Hello, world and above, hello land, world hero");

    let mut word_vector = WordVector::new();
    
    let mut idx = 0;
    loop {
        idx = sentence.next_word(idx);

        match sentence.v_words.last() {
            Some(word_token) => word_vector.add_word(word_token.word.as_str()),
            None => continue
        };
        println!("Word: {:?}", sentence.v_words.last());

        if idx == sentence.len() {
            break;
        }
    }
    println!("word vector");
    for w in word_vector.words {
        println!("{:?}", w)
    }
}



