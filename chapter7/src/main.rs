extern crate chapter7;

fn main() {
    chapter7::client::connect();

    a::series::of::nested_modules();

    use a::series::of;
    of::nested_modules();

    use a::series::of::nested_modules;
    nested_modules();

    use TrafficLight::{Red, Yellow};
    // bring all items into scope
    // use TrafficLight::*;
    let _red = Red;
    let _yellow = Yellow;
    // Green is not in scope;
    // let _green = Green;
    let _green = TrafficLight::Green;
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}
