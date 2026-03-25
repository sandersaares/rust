//@ check-pass
// Regression test: Self::Bounds on GAT parameter should not cause E0276.
// The impl's GAT bounds must be properly expanded so that Arc<T>/Rc<T>
// requirements match the trait definition.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::sync::Arc;
use std::rc::Rc;

trait SmartPtr<T> {
    fn new(val: T) -> Self;
}

impl<T: Send + Sync + 'static> SmartPtr<T> for Arc<T> {
    fn new(val: T) -> Self { Arc::new(val) }
}

impl<T: 'static> SmartPtr<T> for Rc<T> {
    fn new(val: T) -> Self { Rc::new(val) }
}

trait Runtime {
    trait Bounds;
    type Ptr<T: Self::Bounds>: SmartPtr<T>;
}

struct Multi;
impl Runtime for Multi {
    trait Bounds = Send + Sync + 'static;
    type Ptr<T: Self::Bounds> = Arc<T>;
}

struct Single;
impl Runtime for Single {
    trait Bounds = 'static;
    type Ptr<T: Self::Bounds> = Rc<T>;
}

fn make_ptr<R: Runtime, T: R::Bounds>(val: T) -> R::Ptr<T> {
    SmartPtr::new(val)
}

fn main() {
    let _: Arc<i32> = make_ptr::<Multi, _>(42);
    let _: Rc<String> = make_ptr::<Single, _>("hello".to_string());
}
