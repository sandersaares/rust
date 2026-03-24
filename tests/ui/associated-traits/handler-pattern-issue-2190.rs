//@ check-pass
// rust-lang/rfcs#2190 original example — the Handler pattern.
// Tests the core motivating use case from the issue body.

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Handler {
    trait Arg;
    fn handle(&self);
}

struct MyHandler;
impl Handler for MyHandler {
    trait Arg = IntoIterator<Item = i32>;
    fn handle(&self) {}
}

// Generic over implementations of Handler (from OP's example)
fn example_generic_helper<HandlerImpl: Handler, ArgImpl: HandlerImpl::Arg>(
    handler: &HandlerImpl,
    _args: Vec<ArgImpl>,
) {
    handler.handle();
}

// Using UFCS for the same pattern
fn ufcs_helper<H: Handler, A: <H as Handler>::Arg>(_handler: &H, _arg: A) {}

// Combining associated trait bound with concrete trait in where clause
fn combined_helper<H: Handler, A>(handler: &H, _arg: A)
where
    A: H::Arg + Clone,
{
    handler.handle();
}

fn main() {
    let handler = MyHandler;
    // Vec<i32> implements IntoIterator<Item = i32> ✓
    example_generic_helper(&handler, vec![vec![1, 2], vec![3, 4]]);
    ufcs_helper(&handler, vec![5, 6]);
    combined_helper(&handler, vec![7, 8]);
}
