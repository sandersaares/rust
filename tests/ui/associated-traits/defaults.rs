// Test default associated traits.
//@ ignore-test: not yet implemented (associated_traits)

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

fn main() {
    // UsesDefault::Bar is Send
    needs_bar(UsesDefault, 42i32);
    // OverridesDefault::Bar is Clone
    needs_bar(OverridesDefault, String::from("hello"));
}
