fn main() {
    println!("Hello, world!");

    // entire instance must be mutable.
    // Rust does'nt allow to mark only certain fields as mutable.
    let mut user1 = User::_new();
    println!("{}'s email is {}", user1.username, user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("{}'s email is {}", user1.username, user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser567"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        // use struct update syntax
        ..user1
    };

    let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
    let rect1 = Rectangle::new(30, 50);
    println!("The area of the rectanble is {}", rect1.area());
    // println!("rect1 is {}", rect1);
    // using Debug formatter by :?
    // add annotation #[derive(Debug)] to just before the struct definition
    println!("rect1 is {:?}", rect1);
    // prity-print by adding # between : and ?.
    println!("rect1 is {:#?}", rect1);

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    // compile error!
    // p1.distance(p2);

    // automatic referencing
    // Rust automatically adds in &, &mut, or *.
    // distance method's receiver is &self -> &Point.
    p1.distance(&p2);
    (&p1).distance(&p2);

    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn _new() -> User {
        User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            sign_in_count: 1,
            active: true,
        }
    }

    fn new(email: String, username: String) -> User {
        User {
            // email: email,
            email,
            // username: username,
            username,
            sign_in_count: 1,
            active: true,
        }
    }
}

// struct without field name.
// tuple struct
struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated Functions
    // no self parameter.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle::new(size, size)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn widen(&mut self, n: u32) {
        self.width += n
    }

    fn takes_ownership(self) -> Rectangle {
        self
    }

    fn transform(self, w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: self.width + w,
            height: self.height + h,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);

        f64::sqrt(x_squared + y_squared)
    }
}
