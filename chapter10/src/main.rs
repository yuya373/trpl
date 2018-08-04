fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is `{}`", result);
}
