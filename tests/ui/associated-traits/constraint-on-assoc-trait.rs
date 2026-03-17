// Test that `where T::Bar: Clone` constrains the associated trait.
//@ ignore-test: not yet implemented (associated_traits)

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

struct MyStruct;

impl Foo for MyStruct {
    trait Bar = Send;
}

// T::Bar must have Clone as a supertrait
fn constrained<T: Foo>(_t: T)
where
    T::Bar: Clone,
{
}

fn main() {}
