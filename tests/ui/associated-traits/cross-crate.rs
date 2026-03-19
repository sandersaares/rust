// Test cross-crate associated traits.
//@ check-pass
//@ compile-flags: --crate-type=lib
//@ aux-build: cross-crate-dep.rs

#![feature(associated_traits)]
#![allow(incomplete_features)]

extern crate cross_crate_dep;

use cross_crate_dep::{Container, SyncVec};

fn use_cross_crate<C: Container, E: C::Elem>(_c: C, _e: E) {}

fn test() {
    use_cross_crate(SyncVec, 42i32);
}
