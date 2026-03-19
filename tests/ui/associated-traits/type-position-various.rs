// Test that associated traits cannot be used where types are expected.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
}

fn as_return_type<T: Foo>() -> T::Bar {
    //~^ ERROR associated trait `Bar` cannot be used as a type
    todo!()
}

fn as_parameter<T: Foo>(_x: T::Bar) {
    //~^ ERROR associated trait `Bar` cannot be used as a type
}

struct HasField<T: Foo> {
    field: T::Bar,
    //~^ ERROR associated trait `Bar` cannot be used as a type
}
