// Analize English text and extracts features

use nlp::*;


fn main() {
    let sentence = SentenceToken::from("Hello, world and above, hello land, world hero!");

    println!("word vector");
    for w in sentence.v_words {
        println!("{:?}", w)
    }
}



