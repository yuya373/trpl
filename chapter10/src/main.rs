#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3, y: 10 };
    println!("p.x = {}", p.x()); // p.x is not equal to p.x();

    // p.distance_from_origin(); // only Point<f32> have this method

    let p = Point { x: 3.0, y: 10.0 };
    println!("distance from origin: {:?}", p.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    // println!("p1: {:?}", p1); // p1 and p2 is moved value
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
