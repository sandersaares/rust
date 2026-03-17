// Error: associated trait in impl doesn't satisfy declaration bounds.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

trait Foo {
    trait Bar: Clone; // associated trait must be a subtrait of Clone
}

struct Bad;

impl Foo for Bad {
    trait Bar = Send; //~ ERROR the trait bound `Send: Clone` is not satisfied
}

fn main() {}
