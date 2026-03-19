// Test that defaults can be overridden and the override is used.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait HasDefault {
    trait Bound = Send; // default
}

struct UsesDefault;
impl HasDefault for UsesDefault {} // uses Send

struct OverridesDefault;
impl HasDefault for OverridesDefault {
    trait Bound = Clone; // override to Clone
}

fn with_default<T: HasDefault, B: T::Bound>(_t: T, _b: B) {}

fn test() {
    with_default(UsesDefault, 42i32); // i32: Send ✓
    with_default(OverridesDefault, String::from("hi")); // String: Clone ✓
}
