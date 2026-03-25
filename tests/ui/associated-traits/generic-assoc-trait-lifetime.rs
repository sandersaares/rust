// Test generic associated traits with lifetime parameters.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Borrowable {
    trait BorrowConstraint<'a>;
}

struct MyData;
impl Borrowable for MyData {
    trait BorrowConstraint<'a> = Send;
}

fn borrow_it<'a, B: Borrowable, C: B::BorrowConstraint<'a>>(_b: &'a B, _c: C) {}
