// Test associated traits with trait inheritance chains.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

// Base trait with associated trait
trait Base {
    trait Constraint;
}

// Sub-trait inheriting the associated trait
trait Extended: Base {
    fn do_work(&self);
}

// Impl for a concrete type
struct Worker;
impl Base for Worker {
    trait Constraint = Send + Sync;
}
impl Extended for Worker {
    fn do_work(&self) {}
}

// Using the associated trait from the supertrait
fn spawn_work<E: Extended, W: E::Constraint>(worker: E, _witness: W) {
    worker.do_work();
}
