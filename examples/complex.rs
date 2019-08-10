#[macro_use]
extern crate attr_rs;

#[derive(Debug)]
struct Cat {
    name: String,
    age: u64,
}

#[attr_reader(name, cat, age)]
struct Person<'a, S> {
    name: S,
    cat: &'a Cat,
    age: u64,
}

impl<'a, S> Person<'a, S> {
    pub fn new(name: S, cat: &'a Cat, age: u64) -> Self {
        Self { name, cat, age }
    }
}

fn main() {
    let c = Cat {
        name: String::from("Mittens"),
        age: 2,
    };
    let p = Person::new("Nick", &c, 21);
    println!("Name = {}", p.get_name());
    println!("Cat = {:?}", p.get_cat());
    println!("Age = {}", p.get_age());
}
