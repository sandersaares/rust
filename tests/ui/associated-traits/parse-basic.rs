// Basic parsing of associated trait declarations and implementations.
// Verifies the parser accepts `trait` items in trait/impl bodies.
//@ compile-flags: --crate-type=lib
// The compiler will ICE during HIR lowering since that's not implemented yet,
// but the parsing itself should succeed. We mark this as known-bug for now.
//@ known-bug: #99999

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

struct MyStruct;

impl Foo for MyStruct {
    trait Bar = Send;
}
