#[macro_use]
extern crate attr;

#[attr_accessor(name, age)]
struct Cat {
    name: String,
    age: u64,
}

impl Cat {
    pub fn new(name: String, age: u64) -> Self {
        Cat { name, age }
    }
}

fn main() {
    let mut c = Cat::new(String::from("Phil"), 4);
    println!("Name: {}\nAge: {}", c.get_name(), c.get_age());
    c.set_name(String::from("Mittens"));
    c.set_age(6);
    println!("Name: {}\nAge: {}", c.get_name(), c.get_age());
}
