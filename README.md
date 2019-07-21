# Attr-rs

<img src="https://www.rustacean.net/assets/cuddlyferris.svg" alt="Ferris" width="48px" height="48px"><img src="https://img.icons8.com/officel/40/000000/hearts.png"><img src="https://img.icons8.com/color/48/000000/ruby-programming-language.png">

A Rust library focused on implementing one of the **BEST** features in Ruby `attr_reader`, `attr_writer`, and `attr_accessor` using procedural macros in Rust. The main goal being to reduce the common getter and setter methods with a single macro to reduce time spent on boilerplate and allow for more time to spend tackling more interesting problems!

`attr_reader` - Generate **ONLY** getter methods for provided fields

`attr_writer` - Generate **ONLY** setter methods for provided fields

`attr_accessor` - Generate **BOTH** getter and setter methods for provided fields

These `3` macros are Attribute macros and follow the general format of `attr_reader(field1, field2, field3, ...)` and must be placed before/on top of a `struct`

_The Comma separated list is unnecessary, spaces are fine as well as any form of punctuation that the Rust compiler allows for_

### Why

Recently, I've dabbled with Ruby, at Ohio State University in CSE3901, and really loved the `attr` macros that it had built-in the language to prevent the definition of basic `getter` and `setter` methods for any class. It's the small things like this that make a language **GREAT** and I really miss this feature when writing code in other languages. So why not attempt to bring this feature to Rust `struct`!

I'm _lazy_, I'll admit it :sweat_smile:

I don't want to write these super simplistic methods for classes, I'd rather focus on the interesting problem that I seek to tackle with the class rather than the boring, mundane code that is required for the class to function and that's the goal of this crate to increase productivity and decrease bloated source code with the basic methods that are necessary :+1:

### Example

#### Without Attr

```rust
struct Person {
	name: String,
    age: u64,
}

impl Person {
	pub fn new(name: String, age: u64) -> Self {
    	Self { name, age }
    }

    pub fn get_name(&self) -> &str {
    	self.name
    }

    pub fn get_age(&self) -> u64 {
    	self.age
    }

    pub fn set_name(&mut self, name: String) {
    	self.name = name;
    }

    pub fn set_age(&mut self, age: u64) {
    	self.age = age;
    }
}
```

#### With Attr

```rust
#[macro_use]
extern crate attr;

#[attr_accessor(name, age)]
struct Person {
	name: String,
    age: u64,
}

impl Person {
	pub fn new(name: String, age: u64) -> Self {
    	Self { name, age }
    }
}
```

Add to your `Cargo.toml`

```toml
[dependencies]
attr = "0.1.0"
```

### Errors

#### :x: Duplicate attributes

In Ruby the `attr` macros are provided `Symbols` which are required to be unique and therefore this crate seeks to emulate that behavior, mostly because it just makes sense, and therefore will error on duplicate fields.

#### :x: Unknown attribute

Creating getters and setters for an nonexistent field in a `struct` makes no sense.

#### :x: Provided attribute isn't an Identifier

Providing a `Literal` like `6` to create a getters and setters for makes no sense and neither does providing a `Group` like `(Hello)`.

_All Errors are compile time errors_

### Contributions

All are welcome and appreciated :smile:

Please open an issue for any feature requests, and open a pull request for any bug fixes

### License

[MIT](/LICENSE)

Thanks to [Icons8](https://icons8.com/) for the Ruby and Heart Icon :heart:
