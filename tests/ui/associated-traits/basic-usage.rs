// Basic end-to-end usage: declare, implement, use as a bound.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Container {
    trait ElementConstraint;
}

struct SyncContainer;

impl Container for SyncContainer {
    trait ElementConstraint = Send;
}

fn process<C: Container, E: C::ElementConstraint>(_c: C, _e: E) {}

fn test() {
    let c = SyncContainer;
    let e: i32 = 42;
    process(c, e); // i32: Send, so this should work
}
