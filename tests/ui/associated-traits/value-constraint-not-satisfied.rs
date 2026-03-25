// Test that `where C::Elem: Debug` rejects impls whose value doesn't include Debug.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

use std::fmt::Debug;

trait Container {
    trait Elem;
}

struct SendOnly;
impl Container for SendOnly {
    trait Elem = Send; // does NOT include Debug
}

fn need_debug<C: Container, T: C::Elem>(_x: T)
where
    C::Elem: Debug,
{}

fn test_fails() {
    need_debug::<SendOnly, i32>(42);
    //~^ ERROR
}
