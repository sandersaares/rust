//@ check-pass
// Method parameter bounded by Self::Bounds does not substitute the concrete
// value in the impl body. The compiler sees `T: Self::Bounds` but does not
// recognize that `Self::Bounds = Send + Sync + 'static` means `T: 'static`.
//
// Expected: compiles successfully — T: Self::Bounds in the impl of Runtime
// for Multi gives T: Send + Sync + 'static, which satisfies all requirements.
//
// Actual: E0310 "the parameter type `T` may not live long enough"

#![feature(associated_traits)]

trait Runtime {
    trait Bounds;

    fn do_something<T: Self::Bounds>() -> Box<T>
    where
        T: Default;
}

struct Multi;
impl Runtime for Multi {
    trait Bounds = Send + Sync + 'static;

    // E0310: inside this body, T should be known as Send + Sync + 'static,
    // but the compiler doesn't substitute Self::Bounds with its concrete value.
    fn do_something<T: Self::Bounds>() -> Box<T>
    where
        T: Default,
    {
        // Box<T> requires T: Sized (OK) but the body uses T which needs 'static
        // for some operation. The 'static comes from Self::Bounds = ... + 'static.
        let val: Box<dyn std::any::Any> = Box::new(T::default()); // needs T: 'static
        let _ = val;
        Box::new(T::default())
    }
}

fn main() {}
