// Test different impls providing different associated traits.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

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

fn main() {
    // With TypeA, B must be Send
    use_bound(TypeA, 42i32);
    // With TypeB, B must be Clone
    use_bound(TypeB, String::from("hello"));
}
