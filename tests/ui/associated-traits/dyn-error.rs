// Test that dyn with associated traits produces a clear error.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

fn with_dyn<T: Foo>(_x: &dyn T::Bar) {}
//~^ ERROR associated traits cannot be used with `dyn`
//~| ERROR at least one trait is required for an object type
