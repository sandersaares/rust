// Test different impls providing different associated traits.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Constraint {
    trait Bound;
}

struct TypeA;
struct TypeB;

impl Constraint for TypeA {
    trait Bound = Send;
}

impl Constraint for TypeB {
    trait Bound = Clone;
}

fn use_bound<T: Constraint, B: T::Bound>(_t: T, _b: B) {}

fn test() {
    // With TypeA, B must be Send
    use_bound(TypeA, 42i32);
    // With TypeB, B must be Clone
    use_bound(TypeB, String::from("hello"));
}
