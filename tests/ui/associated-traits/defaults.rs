// Test default associated traits.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Foo {
    trait Bar = Send; // default: Bar is Send
}

struct UsesDefault;

impl Foo for UsesDefault {
    // No `trait Bar = ...;` — uses default Send
}

struct OverridesDefault;

impl Foo for OverridesDefault {
    trait Bar = Clone; // Override the default
}

fn needs_bar<T: Foo, B: T::Bar>(_t: T, _b: B) {}

fn test() {
    // UsesDefault::Bar is Send
    needs_bar(UsesDefault, 42i32);
    // OverridesDefault::Bar is Clone
    needs_bar(OverridesDefault, String::from("hello"));
}
