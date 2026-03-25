//@ check-pass
// GAT with both a Self::Bounds parameter bound AND an output bound referencing
// Self::Bounds. The compiler should substitute the concrete associated-trait
// value into both positions.
//
// Expected: compiles successfully — T: Self::Bounds provides the concrete
// bounds that make Ptr<T>: Wrapper<T> + Self::Bounds hold.
//
// Actual: E0276 "impl has stricter requirements than trait"

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
    // Output bound referencing Self::Bounds together with GAT parameter bound
    type Ptr<T: Self::Bounds>: Wrapper<T> + Self::Bounds;
}

struct Multi;
impl Runtime for Multi {
    trait Bounds = Send + Sync + 'static;
    // Should work: Arc<T> is Send + Sync + 'static when T is,
    // and Arc<T>: Wrapper<T> when T: Send + Sync + 'static.
    type Ptr<T: Self::Bounds> = Arc<T>;
}

struct Single;
impl Runtime for Single {
    trait Bounds = 'static;
    type Ptr<T: Self::Bounds> = Rc<T>;
}

fn main() {}
