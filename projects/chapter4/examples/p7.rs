type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice() // explicit ummutable ref to strings
}


fn main() {
    let words = vec!["hello".to_string()]; // is it because we want words to have heap stored stuff instead of string literal slice?
    let d = new_document(words);

    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    assert!(!get_words(&d).contains(&"world".into()));
}