// Associated trait used in type position resolves to a placeholder.
// This is a known limitation — ideally it would produce a warning.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
}

// T::Bar in type position resolves to a projection type.
// For generic T, the projection stays abstract.
// Future work: emit a warning here.
fn usage<T: Foo>() {
    let _x: T::Bar = todo!();
}
