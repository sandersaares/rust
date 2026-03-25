// Test that providing a concrete value that satisfies bounds works.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

// Clone is a subtrait of Clone (trivially)
trait Foo { trait Bar: Clone; }
struct S1;
impl Foo for S1 { trait Bar = Clone; }

// Send + Sync + Clone satisfies Clone
trait Baz { trait Elem: Clone; }
struct S2;
impl Baz for S2 { trait Elem = Clone + Send; }
