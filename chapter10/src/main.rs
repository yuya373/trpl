#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point_<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    println!("integer: {:?}", integer);

    let float = Point { x: 1.0, y: 4.0 };
    println!("float: {:?}", float);

    // both x and y are type T
    // let wont_work = Point { x: 1, y: 3.0 };

    let work = Point_ { x: 1, y: 3.0 };
    println!("two generic type: {:?}", work);
}
