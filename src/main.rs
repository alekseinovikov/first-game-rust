#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name: name, age: age }
    }
}

fn main() {
    let leha = Person::new("Leha".to_owned(), 30);
    println!("{:#?}", leha);
}
