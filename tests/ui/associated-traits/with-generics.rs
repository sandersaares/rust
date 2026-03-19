// Test associated traits with generic trait parameters.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Processor<T> {
    trait Constraint;
}

struct StringProcessor;

impl Processor<String> for StringProcessor {
    trait Constraint = Clone;
}

impl Processor<i32> for StringProcessor {
    trait Constraint = Send;
}

fn process<P: Processor<String>, C: P::Constraint>(_p: P, _c: C) {}
