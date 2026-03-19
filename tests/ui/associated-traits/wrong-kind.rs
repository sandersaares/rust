// Test that an impl providing `type` for a `trait` item is rejected.
// Trait expects `trait Bar;`, impl provides `type Bar = u32;`.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
}

struct S;

impl Foo for S {
    //~^ ERROR not all trait items implemented, missing: `Bar`
    type Bar = u32;
    //~^ ERROR item `Bar` is an associated type, which doesn't match its trait `Foo`
}
