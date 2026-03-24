//@ check-pass
// rust-lang/rfcs#2190 PointerFamily pattern (kennytm/AndreiCravtov).
// Associated traits restrict what types a GAT can accept.

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait PointerFamily {
    trait Bounds;
    type Pointer<T>;
}

// Arc-like family: requires Send + Sync
struct ArcFamily;
impl PointerFamily for ArcFamily {
    trait Bounds = Send + Sync;
    type Pointer<T> = std::sync::Arc<T>;
}

// Rc-like family: no extra bounds
struct RcFamily;
impl PointerFamily for RcFamily {
    trait Bounds = Clone;
    type Pointer<T> = std::rc::Rc<T>;
}

// Generic function that uses both associated trait and GAT
fn wrap_value<F: PointerFamily, T: F::Bounds>(val: T) -> F::Pointer<T>
where
    F::Pointer<T>: From<T>,
{
    F::Pointer::<T>::from(val)
}

fn main() {
    let _arc: std::sync::Arc<i32> = wrap_value::<ArcFamily, _>(42);
    let _rc: std::rc::Rc<String> = wrap_value::<RcFamily, _>(String::from("hello"));
}
