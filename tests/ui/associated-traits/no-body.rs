// Test disambiguation: associated traits cannot have bodies.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Outer {
    trait Inner { fn foo(&self); }
    //~^ ERROR associated traits cannot have a body
}
