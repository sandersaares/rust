// Test multiple value constraints on the same associated trait.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

use std::fmt::{Debug, Display};

trait Container {
    trait Elem;
}

struct FullContainer;
impl Container for FullContainer {
    trait Elem = Debug + Display + Send;
}

fn need_both<C: Container, T: C::Elem>(x: T) -> String
where
    C::Elem: Debug,
    C::Elem: Display,
{
    format!("{} ({:?})", x, x)
}

fn test() {
    need_both::<FullContainer, i32>(42);
}
