// Basic parsing of associated trait declarations and implementations.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

struct MyStruct;

impl Foo for MyStruct {
    trait Bar = Send;
}

fn main() {}
