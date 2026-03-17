// Verifies that `associated_traits` is feature-gated.
// Once parsing is implemented, this test should produce a feature gate error
// when the `associated_traits` feature is not enabled.
//@ ignore-test: not yet implemented (associated_traits)

trait Foo {
    trait Bar;
}

fn main() {}
