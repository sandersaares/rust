//@ check-pass
// Test: dyn coercion in impl body where Self::Bounds expands to
// concrete bounds including Debug + 'static.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::fmt::Debug;

trait Processor {
    trait Bounds;
    fn to_debug<T: Self::Bounds>(item: T) -> Box<dyn Debug>;
}

struct MyProcessor;
impl Processor for MyProcessor {
    trait Bounds = Debug + Send + 'static;
    fn to_debug<T: Self::Bounds>(item: T) -> Box<dyn Debug> {
        // In impl body, Self::Bounds expands to Debug + Send + 'static
        Box::new(item)
    }
}

fn main() {
    let _: Box<dyn Debug> = MyProcessor::to_debug(42i32);
}
