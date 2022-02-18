use std::thread;

fn main() {
    let b = String::from("Hello");
    let a = &b[..];
    thread::spawn(move || {
        println!("{}", a);
    });
    println!("{}", a);
    println!("Hello, world!");
}
