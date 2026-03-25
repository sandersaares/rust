//@ check-pass
// Regression test: Self::Bounds with 'static used in method body.
// Verifies that lifetime bounds from associated traits are expanded
// into the impl method's param_env so Box<dyn Any> (needs 'static) works.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::any::Any;

trait Runtime {
    trait Bounds;
    fn box_any<T: Self::Bounds + Default>() -> Box<dyn Any>;
}

struct Multi;
impl Runtime for Multi {
    trait Bounds = Send + Sync + 'static;

    fn box_any<T: Self::Bounds + Default>() -> Box<dyn Any> {
        // T: Self::Bounds should expand to T: Send + Sync + 'static
        // which satisfies the 'static requirement for Box<dyn Any>
        Box::new(T::default())
    }
}

struct Single;
impl Runtime for Single {
    trait Bounds = 'static;

    fn box_any<T: Self::Bounds + Default>() -> Box<dyn Any> {
        Box::new(T::default())
    }
}

fn main() {
    let _: Box<dyn Any> = Multi::box_any::<i32>();
    let _: Box<dyn Any> = Single::box_any::<String>();
}
