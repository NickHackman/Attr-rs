#[macro_use]
extern crate attr_rs;

#[attr_accessor(name, age)]
struct Person<S> {
    name: S,
    age: u64,
}

impl<S> Person<S> {
    pub fn new(name: S, age: u64) -> Self {
        Self { name, age }
    }
}

fn main() {
    let mut p = Person::new("Nick", 21);
    println!("{}", p.get_name());
    println!("{}", p.get_age());
    p.set_name("Phil");
    p.set_age(6);
    println!("{}", p.get_name());
    println!("{}", p.get_age());
}
