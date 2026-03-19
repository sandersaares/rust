// Test that associated traits in inherent impls are rejected.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

struct MyStruct;

impl MyStruct {
    trait Bar = Send;
    //~^ ERROR associated traits are not allowed in inherent implementations
}
