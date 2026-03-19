// Test associated trait with supertraits.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Base {
    trait Constraint;
}

trait Extended: Base {
    // Can reference associated traits from supertraits
    fn do_something<T: Self::Constraint>(&self, t: T);
}
