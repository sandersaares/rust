// Test disambiguation: associated traits cannot have bodies.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Outer {
    trait Inner { fn foo(&self); }
    //~^ ERROR associated traits cannot have a body
}
