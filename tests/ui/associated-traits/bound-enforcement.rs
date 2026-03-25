// Test that B: T::Bar actually enforces the trait bound.
// Rc<i32> does NOT implement Send, so this should fail.
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]

trait Foo {
    trait Bar;
}

struct S;
impl Foo for S {
    trait Bar = Send;
}

fn need_send<T: Foo, B: T::Bar>(_b: B) {}

fn test() {
    need_send::<S, std::rc::Rc<i32>>(std::rc::Rc::new(42));
    //~^ ERROR `Rc<i32>` cannot be sent between threads safely
}
