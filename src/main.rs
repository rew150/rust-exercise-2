fn main() {
    println!("Hello, world!");
}

fn argmax_length<'a>(l: &'a str, r: &'a str) -> &'a str {
    if l.len() > r.len() { l } else { r }
}

struct Hello<'a>(&'a str);

trait World<'a> {
    fn say_world(&self, world: &'a str);
}

impl<'a> World<'a> for Hello<'a> {
    fn say_world(&self, world: &'a str) {
        println!("{} {}", self.0, world);
    }
}