//@ check-pass
// GAT parameter bounded by Self::Bounds causes E0276 "impl has stricter
// requirements than trait" even though Self::Bounds substitutes to the exact
// bounds the impl type needs.
//
// The compiler does not substitute the concrete associated-trait value when
// checking GAT parameter bounds in the impl block, treating the abstract
// Self::Bounds as if it imposes no requirements on T.

#![feature(associated_traits)]

use std::sync::Arc;
use std::rc::Rc;

trait Wrapper<T>: Clone {
    fn new(val: T) -> Self;
}

impl<T: Send + Sync + 'static> Wrapper<T> for Arc<T> {
    fn new(val: T) -> Self { Arc::new(val) }
}

impl<T: 'static> Wrapper<T> for Rc<T> {
    fn new(val: T) -> Self { Rc::new(val) }
}

trait Runtime {
    trait Bounds;
    // GAT parameter bounded by the associated trait
    type Ptr<T: Self::Bounds>: Wrapper<T>;
}

struct Multi;
impl Runtime for Multi {
    trait Bounds = Send + Sync + 'static;
    // E0276: "impl has stricter requirements than trait"
    // The compiler should see that T: Self::Bounds = T: Send + Sync + 'static,
    // which is exactly what Arc<T>: Wrapper<T> requires.
    type Ptr<T: Self::Bounds> = Arc<T>;
}

struct Single;
impl Runtime for Single {
    trait Bounds = 'static;
    type Ptr<T: Self::Bounds> = Rc<T>;
}

fn use_it<R: Runtime, T: R::Bounds>(val: T) -> R::Ptr<T> {
    Wrapper::new(val)
}

fn main() {
    let _arc: Arc<i32> = use_it::<Multi, _>(42);
    let _rc: Rc<String> = use_it::<Single, _>(String::from("hello"));
}
