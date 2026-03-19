// Test that associated traits cannot be used where types are expected.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
}

fn as_return_type<T: Foo>() -> T::Bar {
    //~^ ERROR expected type, found trait
    todo!()
}

fn as_parameter<T: Foo>(_x: T::Bar) {
    //~^ ERROR expected type, found trait
}

struct HasField<T: Foo> {
    field: T::Bar,
    //~^ ERROR expected type, found trait
}
