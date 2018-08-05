#[derive(Debug)]
struct ImportantExcept<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept {
        part: first_sentence,
    };
    println!("i: {:?}", i);

    let e;
    {
        let s = String::from("str");
        // s does not live long enough
        let t = s.as_str();
        e = ImportantExcept { part: t };
    }
    println!("e: {:?}", e);
}
