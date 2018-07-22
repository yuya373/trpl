// parameter: slice of i32
fn largest(l: &[i32]) -> i32 {
    let mut largest: i32 = l[0];

    // why `iter` ?
    for &n in l.iter() {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn main() {
    let number_list = vec![35, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
