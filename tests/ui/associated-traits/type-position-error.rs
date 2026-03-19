// Error: associated trait cannot be used as a type.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
}

fn bad<T: Foo>() {
    let _x: T::Bar = todo!();
    //~^ ERROR associated trait `Bar` cannot be used as a type
}
