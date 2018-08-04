fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r); // r would be referencing memory that was deallocated when x went out of scope.
}
