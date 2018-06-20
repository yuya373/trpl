fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 3, y: 2 };
    let m3 = Message::Write(String::from("write!"));
    m1.call();
    m2.call();
    m3.call();

    let some_number = Some(5);
    some_number.map(|n| n + 1);
    println!("Some number: {:?}", some_number);

    let coin = Coin::Penny;
    println!("value in cents: {}", value_in_cents(coin));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("value in cents: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("None + 1: {:?}", none);

    let some_u8_value = Some(0u8);
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    // anonymous struct
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message.call: {:?}", self)
    }
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match x {
    //     None => None,
    //     Some(x) => Some(x + 1),
    // }
    x.map(|x| x + 1)
}
