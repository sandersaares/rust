// Test that `where T::Bar: Clone` is accepted in where clauses.
// Currently, T::Bar projects to a placeholder type, so the bound is
// trivially satisfied. Declaration bounds (trait Bar: Clone) provide
// the real enforcement at the impl site.
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

// T::Bar in a where clause is accepted. The bound operates on the
// projection type (currently a placeholder). Declaration bounds on
// the trait provide the real enforcement.
fn constrained<T: Foo>(_t: T)
where
    T::Bar: Clone,
{
}
