// The first rule is that each parameter that is a reference gets its own lifetime parameter.
// The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // first lifetime elision rule
    fn level(&self) -> i32 {
        3
    }

    // third lifetime elision rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let p = "part";
    let e = ImportantExcerpt { part: p };

    println!("level: {}", e.level());
}
