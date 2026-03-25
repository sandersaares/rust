//@ check-pass
// Test that associated traits can be used alongside GATs.
// This is the "PointerFamily" / "Family" pattern from RFC #2190
// (proposed by kennytm and AndreiCravtov).

#![feature(associated_traits)]

// Associated trait used alongside GATs in the same trait
trait Family {
    trait Bounds;
    type Of<T>;
}

struct CloneFamily;
impl Family for CloneFamily {
    trait Bounds = Clone;
    type Of<T> = Vec<T>;
}

struct SendFamily;
impl Family for SendFamily {
    trait Bounds = Send;
    type Of<T> = Option<T>;
}

// The associated trait constrains a type parameter at the use site
fn use_family<F: Family, T: F::Bounds>(_val: T) -> F::Of<T> {
    todo!()
}

// Container with associated trait and GAT
trait Container {
    trait ElemConstraint;
    type Elem<T>;
}

struct SendContainer;
impl Container for SendContainer {
    trait ElemConstraint = Send;
    type Elem<T> = Option<T>;
}

// Use associated trait to constrain function generics alongside GAT
fn wrap<C: Container, T: C::ElemConstraint>(val: T) -> C::Elem<T> {
    todo!()
}

fn main() {
    let _: <CloneFamily as Family>::Of<i32> = vec![42];
    let _: Option<i32> = <SendFamily as Family>::Of::<i32>::from(Some(1));
}
