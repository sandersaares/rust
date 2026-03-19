// Test generic associated traits — basic usage.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

// Type parameter
trait Container {
    trait Elem<T>;
}

struct MyVec;
impl Container for MyVec {
    trait Elem<T> = Send;
}

fn use_it<C: Container, E: C::Elem<i32>>(_c: C, _e: E) {}
fn test() { use_it(MyVec, 42i32); }

// Multiple type parameters
trait Multi {
    trait Pair<A, B>;
}

struct M;
impl Multi for M {
    trait Pair<A, B> = Send;
}

fn use_multi<T: Multi, P: T::Pair<i32, String>>(_t: T, _p: P) {}
