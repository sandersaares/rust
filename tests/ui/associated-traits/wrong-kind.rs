// Test that an impl can provide `type` for a `trait` item.
// Currently both lower to the same HIR representation (ImplItemKind::Type),
// so the compiler accepts this. Future work: validate kind match.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Foo {
    trait Bar;
}

struct S;

impl Foo for S {
    type Bar = u32; // Accepted: both lower to ImplItemKind::Type
}
