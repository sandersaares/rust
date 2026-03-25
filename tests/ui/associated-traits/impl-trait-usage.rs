// Test that impl Trait with associated traits works.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Container {
    trait Elem;
}

struct MyVec;
impl Container for MyVec {
    trait Elem = Send;
}

// impl T::Elem means "returns something implementing whatever Elem resolves to"
fn make_element<C: Container>() -> impl C::Elem {
    42i32
}
