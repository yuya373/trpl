use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut buf = String::new();

    File::open("hello.txt")?.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    let username = read_username_from_file();
    println!("username: {:?}", username);
}
