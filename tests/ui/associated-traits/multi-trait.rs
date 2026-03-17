// Test multi-trait associated trait values.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

struct Multi;

impl Foo for Multi {
    trait Bar = Send + Clone; // multiple trait bounds
}

fn use_it<T: Foo, B: T::Bar>(_t: T, _b: B) {}

fn main() {
    // B must be both Send and Clone
    use_it(Multi, String::from("hello"));
}
