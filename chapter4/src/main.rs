fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
    // drop(s); // manual management??
    // println!("{}", s); // compile error!
    drop(s.clone());
    println!("{}", s); // not compile error

    let x = 5;
    let y = x;
    println!("y = {}", y);

    // in heap: [h, e, l, l, o]
    // in stack: (ptr: -> h, len: 5, capacity: 5) as s1
    let s1 = String::from("hello");
    // in stack: (ptr: -> h, len: 5, capacity: 5) as s2
    // copied from s1 not copy the data on the heap
    let s2 = s1;

    // error[E0382]: use of moved value: `s1`
    // (ptr: -> h) in s1 is not valid (moved) when `let s2 = s1;`
    // and therefore doesn't need to free when s1 goes out of scope.
    // println!("{}, world!", s1);

    println!("{}, world!", s2);

    let s1 = String::from("hello");
    // deeply copy the heap data of String.
    // may be expensive, if data in heap is large;
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    // s moved.
    takes_ownership(s);
    // compile error! use of moved value;
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // i32 is Copy this is valid.
    println!("in main: x = {}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}", s1);
    // s2 is moved.
    // println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let s1 = String::from("hello");
    // we must return original string (as s2) if want to use it later.
    // using reference instead.
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let len = calculate_length_ref(&s2);
    // we can use s2 by using reference.
    // &s2 syntax create a reference that refers to the value of s2
    // but does not own it
    // call having references as function parameters borrowing.
    println!("The length of '{}' is {}.", s2, len);

    // cannot borrow mutable reference from immutable local variable
    // let s = String::from("hello");
    let mut s = String::from("hello");
    // needs mutable
    // change(&s);
    change(&mut s);

    let s1 = &mut s;
    // only one mutable reference in paticular scope.
    // let s2 = &mut s;
    // The benefit of having this restriction is that Rust can prevent data races at compile time. A data
    // race is similar to a race condition and happens when these three behaviors occur:
    // * Two or more pointers access the same data at the same time.
    // * At least one of the pointers is being used to write to the data.
    // * There’s no mechanism being used to synchronize access to the data.

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope. we can make new reference.
    let r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s;
    // we can create immutable reference more than once.
    let r2 = &s;
    // we cannot create a mutable reference while we have an immutable reference.
    // compile error!
    // let r3 = &mut s;

    let s = String::from("hello world");
    // equal to &s[..5];
    let hello = &(s[0..5]);
    // equal to &s[6..];
    let world = &s[6..(s.len())];

    println!("reference to hello world: {}", &s);
    println!("Slice hello: {}, world: {}", hello, world);

    let mut s = String::from("hello world");
    let slice = first_word(&s);
    println!("The first word of '{}' is '{}'", s, slice);
    // compile error! s is already borrowed as immutable by first_word.
    // s.clear();

    // string literals being stored inside the birnary.
    // type of s is &str, slice pointing to specific point of the binary.
    // &str is an immutable reference.
    let s = "Hello world!";
    println!("The first word of '{}' is '{}'", s, first_word_by_slice(s));

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("The entire Vector is {:?}, slice is {:?}", a, slice);
}

fn takes_ownership(some_string: String) {
    println!("in takes_ownership: {}", some_string);
} // some_string goes out of scope and the backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("in makes_copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
// type of parameter is reference to String.
fn calculate_length_ref(s: &String) -> usize {
    // (*s).len() // dereference not needed??
    s.len()
} // s goes out of scope.
  // but reference does not have ownership of what refers to, nothing happens.

// fn change(some_string: &String) {
//     // compile error!
//     // reference is immutable by default.
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// compile error! dangling pointer
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope and is deallocated. s's memory goes away.

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // ownership is moved out, and nothing is deallocated.

// fn first_word(s: &String) -> usize {
//     // convert String to an array of bytes
//     let bytes = s.as_bytes();

//     // for (i, item) in bytes.iter().enumerate() {
//     //     if *item == b' ' {
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    // slice of entire string;
    &s[..]
}

fn first_word_by_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
