// Test bounds on associated trait declarations.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

trait Foo {
    trait Bar: Clone; // associated trait must be a subtrait of Clone
}

struct Good;

impl Foo for Good {
    trait Bar = Clone; // OK: Clone is a subtrait of Clone
}

fn main() {}
