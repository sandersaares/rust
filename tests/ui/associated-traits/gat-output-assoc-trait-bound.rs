//@ check-pass
// Test: GAT output bound combined with associated trait.
// Ensures both the GAT parameter AND output position correctly
// see the expanded associated trait value.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::sync::Arc;

trait Runtime {
    trait Bounds;
    // Output bound: Ptr<T> must satisfy Self::Bounds too
    type Ptr<T: Self::Bounds>: Clone + Self::Bounds;
}

struct Multi;
impl Runtime for Multi {
    trait Bounds = Send + Sync + 'static;
    // Arc<T>: Clone + Send + Sync + 'static when T: Send + Sync + 'static
    type Ptr<T: Self::Bounds> = Arc<T>;
}

fn use_ptr<R: Runtime, T: R::Bounds>(val: T) -> R::Ptr<T>
where
    R::Ptr<T>: From<T>,
{
    R::Ptr::<T>::from(val)
}

fn main() {
    let ptr: Arc<i32> = use_ptr::<Multi, _>(42);
    let _cloned = ptr.clone();
}
