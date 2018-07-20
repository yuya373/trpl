use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    // let g: u32 = f;
    let f = match f {
        Ok(file) => file,
        // Err(e) => panic!("There was a problem opening file: {:?}", e),
        // why need ref?
        // The ref in the pattern is needed so error is
        // not moved into the guard condition but is merely referenced by it. The reason you use
        // ref to create a reference in a pattern instead of & will be covered in detail in Chapter
        // 18. In short, in the context of a pattern, & matches a reference and gives you its
        // value, but ref matches a value and gives you a reference to it.
        Err(ref e) if e.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(file) => file,
            Err(e) => panic!("{:?}", e),
        },
        Err(e) => {
            println!("{:?}", e.kind());
            println!("{:}", e.to_string());
            panic!("{:?}", e)
        }
    };
    println!("{:?}", f);
}
