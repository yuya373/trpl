#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let some: Option<i64> = Option::Some(3);
    println!("some: {:?}", some);

    let none: Option<String> = Option::None;
    println!("none: {:?}", none);

    let ok: Result<i64, String> = Result::Ok(3);
    println!("ok: {:?}", ok);

    let err: Result<i64, String> = Result::Err(String::from("hoge"));
    println!("err: {:?}", err);
}
