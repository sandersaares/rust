// Test that associated trait bounds are expanded inside impl method bodies
// using an explicit generic parameter instead of `impl Trait`.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Handler {
    trait Arg;
    fn handle<T: Self::Arg>(&self, arg: T) -> i32;
}

struct SumHandler;
impl Handler for SumHandler {
    trait Arg = IntoIterator<Item = i32>;
    fn handle<T: Self::Arg>(&self, arg: T) -> i32 {
        let mut sum = 0i32;
        for x in arg {
            sum += x;
        }
        sum
    }
}
