// Verifies that `associated_traits` is feature-gated.
//@ compile-flags: --crate-type=lib

trait Foo {
    trait Bar; //~ ERROR associated traits are experimental
}

fn main() {}
