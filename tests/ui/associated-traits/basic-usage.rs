// Basic end-to-end usage: declare, implement, use as a bound.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

trait Container {
    trait ElementConstraint;
}

struct SyncContainer;

impl Container for SyncContainer {
    trait ElementConstraint = Send;
}

fn process<C: Container, E: C::ElementConstraint>(_c: C, _e: E) {}

fn main() {
    let c = SyncContainer;
    let e: i32 = 42;
    process(c, e); // i32: Send, so this should work
}
