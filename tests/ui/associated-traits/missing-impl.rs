// Error when impl omits a required associated trait.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

struct MyStruct;

impl Foo for MyStruct {
    //~^ ERROR not all trait items implemented, missing: `Bar`
    // Missing `trait Bar = ...;` — should error
}
