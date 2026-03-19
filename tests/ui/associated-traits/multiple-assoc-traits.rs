// Test that multiple associated traits in a single trait work.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Complex {
    trait ReadBound;
    trait WriteBound;
}

struct MyIO;

impl Complex for MyIO {
    trait ReadBound = Send;
    trait WriteBound = Send + Sync;
}

fn io_op<T: Complex, R: T::ReadBound, W: T::WriteBound>(_t: T, _r: R, _w: W) {}

fn test() {
    io_op(MyIO, 42i32, String::from("hello"));
}
