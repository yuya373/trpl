fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let s1 = string1.to_string();

    let result = longest(&s1, string2);
    println!("The longest string is {}", result);
}
