// Test that associated traits in inherent impls are rejected.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

struct MyStruct;

impl MyStruct {
    trait Bar = Send;
    //~^ ERROR associated traits are not allowed in inherent implementations
}
