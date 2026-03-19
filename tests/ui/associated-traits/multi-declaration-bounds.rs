// Test multiple declaration bounds combined with multi-trait value.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Strict {
    trait Requirement: Send + Sync;
}

struct SafeType;

impl Strict for SafeType {
    // Send + Sync is a subtrait of both Send and Sync
    trait Requirement = Send + Sync;
}
