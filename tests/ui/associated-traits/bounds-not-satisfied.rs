// Error: associated trait in impl doesn't satisfy declaration bounds.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar: Clone; // associated trait must be a subtrait of Clone
}

struct Bad;

impl Foo for Bad {
    trait Bar = Send; //~ ERROR associated trait bound `Bar` is not satisfied
}
