//@ check-pass
// Test that associated types can be bounded by associated traits.
// See RFC #2190 comment by JohnScience about DefaultSpecializationExt.

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Container {
    trait Constraint;
    type Item: Self::Constraint;
}

// Item must satisfy the Constraint (Send)
struct SendContainer;
impl Container for SendContainer {
    trait Constraint = Send;
    type Item = i32; // i32: Send ✓
}

// Item must satisfy Clone
struct CloneContainer;
impl Container for CloneContainer {
    trait Constraint = Clone;
    type Item = String; // String: Clone ✓
}

// Using the pattern
fn get_item<C: Container>(item: C::Item) -> C::Item {
    item
}

fn main() {
    let _: i32 = get_item::<SendContainer>(42);
    let _: String = get_item::<CloneContainer>(String::from("hello"));
}
