// Test with user-defined trait as value.
// Currently, non-auto traits as associated trait values produce errors
// during WF checking because the synthetic bounds use () as the bounded type.
// This is a known limitation that will be fixed when the synthetic predicates
// use the projection type instead.
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
    //~^ ERROR impl has stricter requirements than trait
    //~| ERROR the trait bound `(): MyCustomTrait` is not satisfied
}
