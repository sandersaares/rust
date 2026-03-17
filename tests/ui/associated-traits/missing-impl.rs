// Error when impl omits a required associated trait.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

struct MyStruct;

impl Foo for MyStruct {
    // Missing `trait Bar = ...;` — should error
}

fn main() {}
