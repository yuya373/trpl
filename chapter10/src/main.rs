//“Stack-Only Data: Copy” section in Chapter 4
fn largest<T: std::cmp::PartialOrd + std::marker::Copy>(l: &[T]) -> T {
    let mut largest: T = l[0];

    for &n in l {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn largest_ref<T: std::cmp::PartialOrd>(l: &[T]) -> &T {
    let mut largest = &l[0];

    for n in l {
        if *n > *largest {
            largest = n;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let result = largest_ref(&number_list);
    println!("The largest number is {}", *result);

    let result = largest_ref(&char_list);
    println!("The largest char is {}", *result);
}
