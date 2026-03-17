// Error: cannot use associated trait in type position.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

fn bad<T: Foo>() {
    let _x: T::Bar = todo!(); //~ ERROR expected type, found trait
}

fn main() {}
