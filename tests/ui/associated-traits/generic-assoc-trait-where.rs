// Test generic associated traits with where clauses.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

// Where clause on generic parameter
trait Container {
    trait Elem<T> where T: Send;
}

struct MyVec;
impl Container for MyVec {
    trait Elem<T> = Clone where T: Send;
}

fn use_it<C: Container, E: C::Elem<i32>>(_c: C, _e: E) {}
fn test() { use_it(MyVec, 42i32); }

// Where clause with Self bound (mirrors GAT pattern)
trait Lending {
    trait Item<'a> where Self: 'a;
}

struct Data;
impl Lending for Data {
    trait Item<'a> = Send where Self: 'a;
}
