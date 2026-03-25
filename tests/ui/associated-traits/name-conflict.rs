// Test disambiguation: associated traits vs associated types with same-name.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

// Cannot have both a type and a trait with the same name
trait Conflict {
    type Bar;
    trait Bar; //~ ERROR the name `Bar` is defined multiple times
}
