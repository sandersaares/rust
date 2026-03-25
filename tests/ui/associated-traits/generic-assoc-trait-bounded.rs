// Test generic associated traits with bounded parameters.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Processor {
    trait Handler<T: Clone>;
}

struct MyProc;
impl Processor for MyProc {
    trait Handler<T: Clone> = Send;
}

fn process<P: Processor, H: P::Handler<String>>(_p: P, _h: H) {}
fn test() { process(MyProc, String::from("hi")); }
