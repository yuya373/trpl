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

    let mut s = String::new();
    println!("s: {:?}", s);

    let data = "initial contents";
    let s = data.to_string();

    println!("s: {:?}", s);

    let s = String::from("initial contents");
    println!("s: {:?}", s);

    let mut s = String::from("foo");
    println!("s: {:?}", s);

    s.push_str("bar");
    println!("s: {:?}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1: {:?}", s1);
    println!("s2: {:?}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s: {:?}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here
    println!("s3: {:?}", s3);
    // println!("s1: {:?}", s1); // compile error! use of moved value (s1)
    println!("s2: {:?}", s2);

    let s4 = s3 + &s2;
    // + is `fn add(self, s: &str) -> String { ... }`
    // &s2 is &String. Rust use deref coercion.
    // let s4 = s3 + &(s2[..]);
    println!("s4: {:?}", s4);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {:?}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {:?}", s);
    // format! macro does not take owner ship of any of its parameters.
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    let s1 = String::from("hello");
    // Rust strings do not support indexing.
    // let h = s1[0];

    let s = String::from("Hola");
    println!("{:?} is {:?} bytes long", s, s.len());

    let s = String::from("こんにちわ");
    println!("{:?} is {:?} bytes long", s, s.len());

    let hello = String::from("こんにちわ");
    // thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside 'こ' (bytes 0..3) of `こんにちわ`', libcore/str/mod.rs:2111:5
    // let s = &hello[0..2];
    let s = &hello[0..6];
    println!("s: {:?}", s);

    for c in "こんにちわ".chars() {
        println!("{}", c);
    }

    for b in "こんにちわ".bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores: {:?}", scores);

    let scores_with_team = vec![(String::from("Blue"), 10), (String::from("Yellow"), 50)];
    // ???
    // let scores: HashMap<_, _> = scores_with_team.iter().collect();
    let scores: HashMap<_, _> = scores_with_team.into_iter().collect();
    println!("scores: {:?}", scores);

    let name = String::from("Favorite color");
    let value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(name, value); // name and value are moved here.
    println!("map: {:?}", map);
    // compile error! name and value has been moved.
    // println!("{:?}", name);
    // println!("{:?}", value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("score: {:?}", scores.get("Blue"));
    let team_name = String::from("Blue");
    println!("score: {:?}", scores.get(&team_name));
    println!("above score is {:?}'s", team_name); // `get` using reference as argument
    println!("when key is not exists: {:?}", scores.get("Hoge"));

    // not ordered
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
