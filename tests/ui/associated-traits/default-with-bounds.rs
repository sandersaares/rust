// Test that associated traits with defaults interact correctly with bounds.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

// Default satisfies the declaration — no explicit bounds on the default form
trait WithDefault {
    trait Bound = Clone; // default
}

struct S;
impl WithDefault for S {} // uses default

// Override with multi-trait
struct T;
impl WithDefault for T {
    trait Bound = Clone + Send; // OK
}
