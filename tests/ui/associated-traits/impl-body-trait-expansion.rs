//@ check-pass
// Test: impl body can use Send methods when Self::Bounds = Send.
// Ensures trait bounds (not just lifetimes) are properly expanded.

#![feature(associated_traits)]
#![allow(incomplete_features)]

fn require_send<T: Send>(_: &T) {}
fn require_clone<T: Clone>(_: &T) -> T { unimplemented!() }

trait Container {
    trait ElemBound;
    fn check<T: Self::ElemBound>(val: &T);
}

struct SendContainer;
impl Container for SendContainer {
    trait ElemBound = Send;
    fn check<T: Self::ElemBound>(val: &T) {
        // T: Self::ElemBound should expand to T: Send
        require_send(val);
    }
}

struct CloneContainer;
impl Container for CloneContainer {
    trait ElemBound = Clone;
    fn check<T: Self::ElemBound>(val: &T) {
        // T: Self::ElemBound should expand to T: Clone
        let _copy = require_clone(val);
    }
}

fn main() {
    SendContainer::check(&42i32);
    CloneContainer::check(&String::from("hello"));
}
