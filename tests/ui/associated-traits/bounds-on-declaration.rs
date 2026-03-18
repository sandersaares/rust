// Test bounds on associated trait declarations.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar: Clone; // associated trait must be a subtrait of Clone
}

struct Good;

impl Foo for Good {
    trait Bar = Clone; // OK: Clone is a subtrait of Clone
}
