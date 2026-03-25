//@ check-pass
// Complex combination: generic associated traits + UFCS + bounds + defaults.
// Tests multiple features interacting together.

#![feature(associated_traits)]

// Trait with generic associated trait that has a default
trait Transform {
    trait Input<T>: Clone = Clone;
    trait Output: Send = Send;
}

struct MyTransform;
impl Transform for MyTransform {
    trait Input<T> = Clone + PartialEq;
    // Output keeps the default (Send)
}

// Using both associated traits together with UFCS
fn process<T: Transform, I: <T as Transform>::Input<u32>, O: T::Output>(
    _input: I,
) {}

fn main() {
    // i32: Clone + PartialEq ✓, i32: Send ✓
    process::<MyTransform, i32, i32>(42);
}
