// Test that `dyn Foo` compiles when `Foo` has an associated trait.
// The trait itself is dyn-compatible — only using the associated trait
// in a dyn bound position (e.g. `dyn T::Bar`) is rejected.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Greetable {
    trait Style;
    fn greet(&self) -> &str;
}

struct Formal;
impl Greetable for Formal {
    trait Style = Send;
    fn greet(&self) -> &str { "Good day." }
}

// `dyn Greetable` is fine — the associated trait is simply unused in this context.
fn greet_any(g: &dyn Greetable) -> &str {
    g.greet()
}
