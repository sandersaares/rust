// Trait with both associated types and associated traits.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Mixed {
    type Output;
    trait Constraint;
    fn process(&self) -> Self::Output;
}

struct MyProcessor;

impl Mixed for MyProcessor {
    type Output = i32;
    trait Constraint = Send;
    fn process(&self) -> i32 { 42 }
}

fn use_mixed<M: Mixed, C: M::Constraint>(m: M, _c: C) -> M::Output {
    m.process()
}
