// Test associated trait without feature gate in different positions.
//@ compile-flags: --crate-type=lib

// No #![feature(associated_traits)]

trait InTrait {
    trait Bar; //~ ERROR associated traits are experimental
}

struct S;

impl InTrait for S {
    trait Bar = Send; //~ ERROR associated traits are experimental
}
