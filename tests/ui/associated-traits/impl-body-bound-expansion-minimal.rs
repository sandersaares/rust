// Minimal test: verify the param_env expansion adds IntoIterator bound
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Handler {
    trait Arg;
    fn handle<T: Self::Arg>(&self, arg: T) -> i32;
}

struct SumHandler;
impl Handler for SumHandler {
    trait Arg = IntoIterator<Item = i32>;
    fn handle<T: Self::Arg>(&self, arg: T) -> i32 {
        // Just check that T: IntoIterator is recognized
        fn assert_into_iter<I: IntoIterator>(_: &I) {}
        assert_into_iter(&arg);
        0
    }
}
