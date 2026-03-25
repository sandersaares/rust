//@ check-pass
// Complex combination: associated traits + associated types used together
// in the same trait, with cross-references.

#![feature(associated_traits)]

trait Collection {
    trait ElemConstraint;
    type Elem: Self::ElemConstraint;
    type Iter: Iterator<Item = Self::Elem>;

    fn iter(&self) -> Self::Iter;
}

struct SendVec<T>(Vec<T>);

impl<T: Send + Clone> Collection for SendVec<T> {
    trait ElemConstraint = Send + Clone;
    type Elem = T;
    type Iter = std::vec::IntoIter<T>;

    fn iter(&self) -> Self::Iter {
        self.0.clone().into_iter()
    }
}

// Using both associated trait and associated type
fn process_collection<C: Collection>(col: &C)
where
    C::Elem: std::fmt::Debug,
{
    // C::ElemConstraint is Send + Clone for SendVec
    // C::Elem satisfies C::ElemConstraint
    // C::Iter yields C::Elem
}

// Generic function constrained by the associated trait
fn send_elems<C: Collection, T: C::ElemConstraint>(_col: &C, _extra: T) {}

fn main() {
    let v = SendVec(vec![1i32, 2, 3]);
    process_collection(&v);
    send_elems(&v, 42i32);
}
