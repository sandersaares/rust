// Error: associated trait cannot be used as a type.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

fn bad<T: Foo>() {
    let _x: T::Bar = todo!();
    //~^ ERROR expected type, found trait
}
