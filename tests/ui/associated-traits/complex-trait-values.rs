//@ check-pass
// Test that associated traits can have complex trait values with
// associated type parameters (e.g., IntoIterator<Item=i32>).
// This is the primary example from the RFC #2190 original proposal.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::fmt::Debug;

trait Handler {
    trait Arg;
}

// Associated trait value with associated type constraints
struct IntHandler;
impl Handler for IntHandler {
    trait Arg = IntoIterator<Item = i32>;
}

// Another complex trait value
struct DebugHandler;
impl Handler for DebugHandler {
    trait Arg = Debug + Clone;
}

// Multiple associated type params
trait Transformer {
    trait Source;
    trait Sink;
}

struct StringTransformer;
impl Transformer for StringTransformer {
    trait Source = IntoIterator<Item = String>;
    trait Sink = Extend<String>;
}

// Usage from the caller side — bounds are enforced correctly
fn handle_int<H: Handler, A: H::Arg>(arg: A)
where
    A: IntoIterator<Item = i32>,
{
    for number in arg {
        let _ = number + 1;
    }
}

fn handle_debug<H: Handler, A: H::Arg + Debug + Clone>(arg: A) {
    let _cloned = arg.clone();
    let _ = format!("{:?}", _cloned);
}

fn main() {
    // Vec<i32> implements IntoIterator<Item = i32>
    handle_int::<IntHandler, _>(vec![1, 2, 3]);

    // &str implements Debug + Clone
    handle_debug::<DebugHandler, _>("hello");
}
