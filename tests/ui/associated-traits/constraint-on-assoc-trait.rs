// Test that associated trait bounds can be used with where clauses.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

struct MyStruct;

impl Foo for MyStruct {
    trait Bar = Send;
}

// Associated traits work in bound position via B: T::Bar syntax.
fn constrained<T: Foo, B: T::Bar>(_t: T, _b: B) {}
