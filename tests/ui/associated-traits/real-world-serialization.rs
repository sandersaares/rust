// Real-world example: a framework where types declare their required
// capabilities via associated traits.
//@ check-pass
//@ compile-flags: --crate-type=lib

#![feature(associated_traits)]
#![allow(incomplete_features)]

// Types declare what thread-safety guarantees they need
trait DataStore {
    trait ThreadSafety;
}

struct InMemoryStore;
impl DataStore for InMemoryStore {
    trait ThreadSafety = Send + Sync; // fully thread-safe
}

struct ThreadLocalStore;
impl DataStore for ThreadLocalStore {
    trait ThreadSafety = Send; // only needs Send
}

// Generic code that operates on any store with the right guarantees
fn transfer_data<S: DataStore, D: S::ThreadSafety>(_store: S, _data: D) {
    // D is guaranteed to satisfy whatever ThreadSafety S requires
}

fn test() {
    transfer_data(InMemoryStore, String::from("data")); // String: Send + Sync
    transfer_data(ThreadLocalStore, 42i32); // i32: Send
}
