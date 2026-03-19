// Associated trait on a generic impl.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Processor {
    trait Constraint;
}

// Generic impl — associated trait applies for all T: Clone
impl<T: Clone> Processor for Vec<T> {
    trait Constraint = Send;
}

fn process<P: Processor, C: P::Constraint>(_p: P, _c: C) {}

fn test() {
    process(vec![1, 2, 3], 42i32);
}
