// Test that value constraints allow deriving bounds in generic function bodies.
// When T: C::Elem and C::Elem: Debug, we should be able to use T as Debug.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

use std::fmt::Debug;

trait Container {
    trait Elem;
}

// The value constraint C::Elem: Debug should let us use Debug on T.
fn use_debug_in_body<C: Container, T: C::Elem>(x: &T) -> String
where
    C::Elem: Debug,
{
    format!("{:?}", x)
}
