#[macro_use]
extern crate attr;

#[attr_accessor(name, age)]
struct Person<'a> {
    name: &'a str,
    age: u64,
}

impl<'a> Person<'a> {
    pub fn new(name: &'a str, age: u64) -> Self {
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
