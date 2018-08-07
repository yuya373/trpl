// 1. fn first_word<'a>(s: &'a str) -> &str {
// 2. fn first_word<'a>(s: &'a str) -> &'a str {
// Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let sentence = "This is long sentence";
    let word = first_word(sentence);
    println!("word: {}", word);
}
