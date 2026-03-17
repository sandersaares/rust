// Basic parsing of associated trait declarations and implementations.
// Verifies the parser accepts `trait` items in trait/impl bodies
// and they lower to HIR without errors.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
}

struct MyStruct;

impl Foo for MyStruct {
    trait Bar = Send;
}
