//@ check-pass
// Test that associated traits can have lifetime bounds as values.
// See RFC issue #2190 comment by kennytm.

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Container {
    trait Bounds;
}

// 'static as associated trait value
struct StaticContainer;
impl Container for StaticContainer {
    trait Bounds = 'static;
}

// Lifetime + trait combination
trait MixedContainer {
    trait Bounds;
}

struct SendStaticContainer;
impl MixedContainer for SendStaticContainer {
    trait Bounds = Send + 'static;
}

// Using lifetime-bounded associated trait in a function
fn use_static<C: Container, T: C::Bounds>(_t: T) {}
fn use_mixed<C: MixedContainer, T: C::Bounds>(_t: T) {}

fn main() {
    // 'static reference satisfies 'static bound
    use_static::<StaticContainer, &'static str>(&"hello");

    // i32 is Send + 'static
    use_mixed::<SendStaticContainer, i32>(42);
}
