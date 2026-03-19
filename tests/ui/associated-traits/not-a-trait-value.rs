// Test that providing a non-trait value produces an error.
// `u32` is a type, not a trait.

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
}

struct S;

impl Foo for S {
    trait Bar = u32; //~ ERROR expected trait, found builtin type `u32` [E0404]
}

fn main() {}
