// Test the suggestion for missing associated trait in impl.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
    trait Baz;
}

struct S;

impl Foo for S {
    //~^ ERROR not all trait items implemented, missing: `Bar`, `Baz`
    // Missing both associated traits
}
