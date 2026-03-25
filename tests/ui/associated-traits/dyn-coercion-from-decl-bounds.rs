//@ check-pass
// Test: declaration bounds with 'static enable dyn coercion.
// This is the core use case: T: C::Elem where trait Elem: Debug + 'static
// should allow Box<dyn Debug> without additional where clauses.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::any::Any;
use std::fmt::Debug;

trait Serializable {
    trait Format: Debug + Send + 'static;
}

struct JsonFormat;
impl Serializable for JsonFormat {
    trait Format = Debug + Send + Sync + 'static;
}

// Declaration bounds provide 'static + Debug — enough for Box<dyn Any> and Box<dyn Debug>
fn serialize_to_any<S: Serializable, T: S::Format>(item: T) -> Box<dyn Any> {
    Box::new(item)
}

fn serialize_to_debug<S: Serializable, T: S::Format>(item: T) -> Box<dyn Debug> {
    Box::new(item)
}

fn main() {
    let _: Box<dyn Any> = serialize_to_any::<JsonFormat, _>(42i32);
    let _: Box<dyn Debug> = serialize_to_debug::<JsonFormat, _>("hello");
}
