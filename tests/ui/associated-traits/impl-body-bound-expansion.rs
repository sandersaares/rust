// Test that associated trait bounds are expanded inside impl method bodies.
// When `Self::Arg` is known to be `IntoIterator<Item = i32>` inside the impl,
// the compiler should let us call IntoIterator methods on `impl Self::Arg`.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Handler {
    trait Arg;
    fn handle(&self, arg: impl Self::Arg) -> i32;
}

struct SumHandler;
impl Handler for SumHandler {
    trait Arg = IntoIterator<Item = i32>;
    fn handle(&self, arg: impl Self::Arg) -> i32 {
        let mut sum = 0i32;
        for x in arg {
            sum += x;
        }
        sum
    }
}
