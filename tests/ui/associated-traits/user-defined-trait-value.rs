// Test with user-defined trait as associated trait value.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait MyCustomTrait {
    fn do_thing(&self);
}

trait HasCustom {
    trait Behavior;
}

struct Concrete;

impl HasCustom for Concrete {
    trait Behavior = MyCustomTrait;
}

fn use_custom<T: HasCustom, B: T::Behavior>(_t: T, _b: B) {}
