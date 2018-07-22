// parameter: slice of i32
fn largest_i32(l: &[i32]) -> i32 {
    let mut largest: i32 = l[0];

    // why `iter` ?
    for &n in l.iter() {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn largest_char(l: &[char]) -> char {
    let mut largest = l[0];

    for &n in l {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn main() {
    let number_list = vec![35, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');
}
