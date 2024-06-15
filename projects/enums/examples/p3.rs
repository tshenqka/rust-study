fn find_word(word: &str, sentence: &str) -> Option<usize> {
    sentence.find(word)
}

fn main() {
    let sentence = "The quick fox jumps over the lazy dog";

    match find_word("fox", sentence) {
        Some(indx) => println!("Found 'fox' at index {}", indx),
        None => println!("'fox' not found in the sentence"),
    }
}