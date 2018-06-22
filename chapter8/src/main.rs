fn main() {
    println!("Hello, world!");
    let _v: Vec<i32> = Vec::new();
    // infer type of _v
    let _v = vec![1, 2, 3];

    // infer type of v from v.push(5);
    let mut v = Vec::new();
    // need mut
    // let v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let _third: &i32 = &v[2];
    let _third: Option<&i32> = v.get(3);

    // panic!
    // let does_not_exists = &v[100];
    let does_not_exists = v.get(100);
    println!("v.get(100): {:?}", does_not_exists);

    let mut v = vec![1, 2, 3, 4, 5];
    let _first = &v[0];
    // cannot borrow v as mutable, v is already borrowed as immutable
    // pushing new element to vector may needs reallocation
    // and copying old elements to new space, old reference can be invalid.
    // v.push(6);

    let v = vec![100, 32, 57];
    for i in v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.0),
    ];
    println!("row: {:?}", row);
}
