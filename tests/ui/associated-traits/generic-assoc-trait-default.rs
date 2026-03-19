// Test generic associated traits with defaults.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Container {
    trait Elem<T> = Send;  // default generic associated trait
}

struct MyVec;
impl Container for MyVec {} // uses default

struct Custom;
impl Container for Custom {
    trait Elem<T> = Clone; // override
}
