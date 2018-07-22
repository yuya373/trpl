#[derive(Debug)]
pub struct Guess {
    // It’s important that the value field be private so code using the Guess struct is not allowed to set value directly
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // test `value`
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    // getter
    pub fn value(&self) -> u32 {
        self.value
    }
}

fn important_func(guess: &Guess) {
    // do something important with value...
    println!("Guess: {:?}", guess);
}

fn main() {
    // panic!
    // let guess = Guess::new(1000);
    // important_func(&guess);
    let guess = Guess::new(1);
    important_func(&guess);
}
