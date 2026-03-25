//@ check-pass
//@ edition:2021
// rust-lang/rfcs#2190 async runtime agnosticism pattern (nihohit).
// Associated trait controls Send requirements for futures.

#![feature(associated_traits)]

use std::future::Future;

trait Runtime {
    trait FutureConstraint;
}

// Tokio-like: requires Send + 'static
struct TokioLike;
impl Runtime for TokioLike {
    trait FutureConstraint = Send + 'static;
}

// Single-threaded: only requires 'static
struct LocalRuntime;
impl Runtime for LocalRuntime {
    trait FutureConstraint = 'static;
}

// Generic spawn function — the constraint comes from the runtime
fn spawn_on<R: Runtime, F: Future<Output = ()> + R::FutureConstraint>(_f: F) {}

// UFCS version for disambiguation
fn spawn_ufcs<R: Runtime, F: Future<Output = ()> + <R as Runtime>::FutureConstraint>(_f: F) {}

fn main() {
    // i32 satisfies Send + 'static
    spawn_on::<TokioLike, _>(async { let _x: i32 = 42; });
    // Also works with UFCS
    spawn_ufcs::<TokioLike, _>(async { let _x: i32 = 42; });
    // Local runtime is more permissive
    spawn_on::<LocalRuntime, _>(async { let _x: i32 = 42; });
}
