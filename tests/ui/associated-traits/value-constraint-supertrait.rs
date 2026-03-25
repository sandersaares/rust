// Test value constraints with supertrait satisfaction.
// If Elem = Clone, and Clone: Debug is NOT true, but if we require Elem: Clone
// (which IS satisfied), that should pass.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Container {
    trait Elem;
}

struct CloneContainer;
impl Container for CloneContainer {
    trait Elem = Clone + Send;
}

// C::Elem: Clone — satisfied because Elem = Clone + Send, and Clone is in Clone's supertraits
fn need_clone<C: Container, T: C::Elem>(_x: T)
where
    C::Elem: Clone,
{}

fn test() {
    need_clone::<CloneContainer, i32>(42);
}
