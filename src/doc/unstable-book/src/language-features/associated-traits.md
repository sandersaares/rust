# `associated_traits`

The tracking issue for this feature is: [#99999]

[#99999]: https://github.com/rust-lang/rust/issues/99999

------------------------

Allows traits to declare **associated traits** — associated items that reference
traits instead of types. Implementors specify which trait(s) their associated
trait resolves to, and consumers can use it as a bound.

```rust
#![feature(associated_traits)]

trait Container {
    trait Elem;
}

struct SyncVec;

impl Container for SyncVec {
    trait Elem = Send;
}

fn process<C: Container, E: C::Elem>(_container: C, _element: E) {
    // E must implement whatever trait C::Elem resolves to.
    // For SyncVec, E must implement Send.
}
```

## Declaration bounds

Associated traits can have bounds that constrain what the implementor may provide:

```rust,compile_fail
#![feature(associated_traits)]

trait Serializable {
    trait Format: Clone; // Format must be a subtrait of Clone
}

struct MyData;

impl Serializable for MyData {
    trait Format = Send; // ERROR: Send is not a subtrait of Clone
}
```

## Defaults

A trait can provide a default associated trait that impls may override:

```rust
#![feature(associated_traits)]

trait Logger {
    trait Filter = Send; // default
}

struct FileLogger;
impl Logger for FileLogger {} // uses default (Send)

struct NetLogger;
impl Logger for NetLogger {
    trait Filter = Send + Sync; // override
}
```

## Multi-trait values

An associated trait can resolve to multiple traits:

```rust
#![feature(associated_traits)]

trait Pipeline {
    trait Constraint;
}

struct SafePipeline;
impl Pipeline for SafePipeline {
    trait Constraint = Send + Sync + Clone;
}
```
