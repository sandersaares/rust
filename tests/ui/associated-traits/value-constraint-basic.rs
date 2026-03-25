// Test that `where C::Elem: OtherTrait` constrains the associated trait value.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

use std::fmt::Debug;

trait Container {
    trait Elem;
}

struct DebugContainer;
impl Container for DebugContainer {
    trait Elem = Debug + Send;
}

struct SendContainer;
impl Container for SendContainer {
    trait Elem = Send;
}

// C::Elem: Debug constrains that the Container's Elem value includes Debug.
// Inside the body, T should be usable as Debug since T: C::Elem and C::Elem: Debug.
fn print_element<C: Container, T: C::Elem>(x: T)
where
    C::Elem: Debug,
{
    println!("{:?}", x);
}

// Concrete call: DebugContainer has Elem = Debug + Send, which includes Debug.
fn test_passes() {
    print_element::<DebugContainer, i32>(42);
}
