//@ check-pass
// Test: dyn coercion with associated traits via explicit where clause.
// When T: C::Elem and an explicit where clause adds T: Debug + 'static,
// Box<dyn Debug> coercion should work.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::fmt::Debug;

trait Container {
    trait Elem;
}

struct MyContainer;
impl Container for MyContainer {
    trait Elem = Debug + Send + 'static;
}

// Explicit where clause allows dyn coercion
fn to_dyn_explicit<C: Container, T: C::Elem + Debug + 'static>(item: T) -> Box<dyn Debug> {
    Box::new(item)
}

fn main() {
    let _: Box<dyn Debug> = to_dyn_explicit::<MyContainer, i32>(42);
}
