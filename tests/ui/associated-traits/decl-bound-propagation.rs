//@ check-pass
// Test: declaration bounds on associated traits propagate to callers.
// When `trait Elem: Debug + 'static` is declared, a function with
// `T: C::Elem` should be able to use T as Debug and create Box<dyn Debug>.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::fmt::Debug;

trait Container {
    trait Elem: Debug + 'static;
}

struct MyContainer;
impl Container for MyContainer {
    trait Elem = Debug + Send + 'static;
}

// Declaration bound propagation: T: C::Elem implies T: Debug + 'static
fn to_dyn<C: Container, T: C::Elem>(item: T) -> Box<dyn Debug> {
    Box::new(item)
}

// Can also use Debug methods directly
fn print_elem<C: Container, T: C::Elem>(item: &T) {
    println!("{:?}", item);
}

// Multiple declaration bounds
trait Processor {
    trait Input: Clone + Debug;
}

struct MyProcessor;
impl Processor for MyProcessor {
    trait Input = Clone + Debug + Send;
}

fn clone_and_debug<P: Processor, T: P::Input>(item: T) -> T {
    println!("{:?}", item);
    item.clone()
}

fn main() {
    let _: Box<dyn Debug> = to_dyn::<MyContainer, i32>(42);
    print_elem::<MyContainer, _>(&42i32);
    let _ = clone_and_debug::<MyProcessor, _>(String::from("hello"));
}
