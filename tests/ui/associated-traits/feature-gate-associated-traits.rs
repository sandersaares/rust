// Verifies that `associated_traits` is feature-gated.
// Without the feature, the parser still parses `trait Bar;` as an associated
// trait but the feature gate check emits an error.
//@ compile-flags: --crate-type=lib
//@ known-bug: #99999

trait Foo {
    trait Bar;
}

fn main() {}
