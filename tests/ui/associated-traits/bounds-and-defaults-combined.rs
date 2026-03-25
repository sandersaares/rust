// Test that associated traits support combined declaration bounds AND defaults.
// Syntax: `trait Elem: Clone = Send + Clone;`
// This means: "impls must provide a subtrait of Clone" (bound) and
// "if no value is given, default to Send + Clone" (default).
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Container {
    // Combined: declaration bound (Clone) + default value (Send + Clone)
    trait Elem: Clone = Send + Clone;
}

struct A;
impl Container for A {
    // Override: Clone + Send satisfies the Clone declaration bound ✓
    trait Elem = Clone + Send;
}

struct B;
impl Container for B {} // Uses default (Send + Clone), which satisfies Clone ✓

// Usage: the bound resolves correctly for both A and B.
fn process<C: Container, E: C::Elem>(_c: C, _e: E) {}
