// Test wrong number of generic arguments on associated trait usage.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Container {
    trait Elem<T>;
}

// Too many args
fn too_many<C: Container, E: C::Elem<i32, i32>>(_c: C, _e: E) {}
//~^ ERROR associated trait takes 1 generic argument but 2 generic arguments were supplied

// Too few args
fn too_few<C: Container, E: C::Elem>(_c: C, _e: E) {}
//~^ ERROR missing generics for associated trait
