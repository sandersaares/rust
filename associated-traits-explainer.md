# Associated Traits: How the Compiler Makes It Work

This document explains how the Rust compiler implements the **associated traits** feature — from parsing your code to actually checking that bounds are satisfied. It also summarizes the key concepts and open questions from the RFC.

If you already know what associated traits *do* (declare trait-level constraints inside a trait, provide concrete values in `impl` blocks, use them as bounds), this document explains what happens *underneath*.

---

## Table of Contents

1. [Quick Refresher](#quick-refresher)
2. [The Compiler Pipeline at a Glance](#the-compiler-pipeline-at-a-glance)
3. [Stage 1: Parsing — Turning Text into Structure](#stage-1-parsing--turning-text-into-structure)
4. [Stage 2: AST Lowering — Connecting the Dots](#stage-2-ast-lowering--connecting-the-dots)
5. [Stage 3: Type Checking — Making Sure It All Adds Up](#stage-3-type-checking--making-sure-it-all-adds-up)
6. [Stage 4: The Trait Solver — Does This Type Actually Satisfy That Bound?](#stage-4-the-trait-solver--does-this-type-actually-satisfy-that-bound)
7. [What Can't You Do (Yet)](#what-cant-you-do-yet)
8. [RFC Concepts Explained](#rfc-concepts-explained)
9. [Open Questions and Their Impact](#open-questions-and-their-impact)

---

## Quick Refresher

Here's the minimal associated traits example. Everything below refers back to this:

```rust
#![feature(associated_traits)]
#![allow(incomplete_features)]

// DECLARATION: "Every Container must say what Elem means"
trait Container {
    trait Elem;
}

// IMPLEMENTATION: "For SyncVec, Elem means Send"
struct SyncVec;
impl Container for SyncVec {
    trait Elem = Send;
}

// USAGE: "E must satisfy whatever C::Elem resolves to"
fn process<C: Container, E: C::Elem>(_c: C, _e: E) {}

fn main() {
    process(SyncVec, 42i32); // i32: Send ✓
}
```

---

## The Compiler Pipeline at a Glance

When the compiler processes this code, it goes through roughly these stages:

```
Source code
  │
  ▼
PARSING ──────────► AST (Abstract Syntax Tree)
  │                   "I see a trait item called Elem"
  ▼
AST LOWERING ─────► HIR (High-level Intermediate Representation)
  │                   "C::Elem in a bound position is an associated trait bound"
  ▼
TYPE CHECKING ────► Predicates and obligations
  │                   "This impl's value satisfies the declaration bounds"
  ▼
TRAIT SOLVING ────► Success or error
                     "i32 satisfies Send (which is what SyncVec::Elem resolves to)"
```

Each stage builds on the previous one. Let's walk through them.

---

## Stage 1: Parsing — Turning Text into Structure

### What happens

The parser reads your source text and produces an **AST** (Abstract Syntax Tree) — a structured representation of the code. For associated traits, the parser recognizes three forms inside a trait or impl body:

| Syntax | Meaning |
|--------|---------|
| `trait Elem;` | Declaration — no default |
| `trait Elem: Clone;` | Declaration with a bound — impls must include Clone |
| `trait Elem = Send + Clone;` | Value (in impl) or default (in trait) || `trait Elem: Clone = Send + Clone;` | Combined: bound (Clone) + default (Send + Clone) |
### How it's stored

The parser creates an `AssocTraitItem` node with these fields:

| Field | What it holds | Example |
|-------|---------------|---------|
| `ident` | The name | `Elem` |
| `generics` | Generic parameters | `<T: Clone>` in `trait Elem<T: Clone>;` |
| `bounds` | Declaration bounds (supertraits) | `Clone` in `trait Elem: Clone;` |
| `value` | The default or impl value | `Send + Clone` in `trait Elem = Send + Clone;` |
| `has_value` | Whether `= ...` was written | `true` or `false` |

### What gets rejected

The parser explicitly rejects bodies:

```rust
trait Elem { } // ERROR: "associated traits cannot have a body;
               //         use 'trait Bar;', 'trait Bar = Send;',
               //         or 'trait Bar: Clone = Send;'"
```

### Feature gating

Unless you write `#![feature(associated_traits)]`, the parser records the span and a later pass emits:

```
error[E0658]: associated traits are experimental
```

Because the feature is marked `incomplete` (not just `unstable`), you also need `#![allow(incomplete_features)]` to suppress an additional warning.

---

## Stage 2: AST Lowering — Connecting the Dots

### What happens

AST lowering transforms the parser's AST into the **HIR** (High-level Intermediate Representation). The HIR is what the type checker actually works with. This stage does something crucial for associated traits: it figures out *where* an associated trait is being *used*.

### The detection problem

Consider this bound:

```rust
fn process<C: Container, E: C::Elem>(_c: C, _e: E) {}
```

When the parser sees `C::Elem` in bound position, it doesn't yet know that `Elem` is an associated trait (as opposed to, say, a regular trait called `C::Elem`). The parser just sees a path.

During AST lowering, the compiler resolves `C` to a type parameter and sees that `Elem` is an *unresolved trailing segment*. It then checks: "Does the trait that `C` is bounded by have an associated item called `Elem` that is a `DefKind::AssocTrait`?" If yes, this path becomes a special HIR node:

```
GenericBound::AssocTraitBound(
    base_type,       // C (the type parameter)
    segment,         // Elem (the associated trait name)
    span,            // source location (for error messages)
    constraint_trait // Some(DefId) if written as <C as Container>::Elem
)
```

### UFCS form

The fully-qualified syntax `<T as Container>::Elem` is also lowered here. In this case, the `constraint_trait` field is populated with the `DefId` of `Container`, so the solver knows exactly which trait to look up:

```rust
fn transfer<T: Readable + Writable, R: <T as Readable>::Constraint>(data: R) {}
//                                      ^^^^^^^^^^^^^^^^^^^^^^^^
//                                      constraint_trait = Some(DefId of Readable)
```

### What gets rejected at this stage

Using an associated trait with `dyn` *directly as a bound* is caught here:

```rust
fn with_dyn<T: Foo>(_x: &dyn T::Bar) {}
// ERROR: "associated traits cannot be used with `dyn`"
```

Note that a trait which *has* associated traits can still be used as `dyn Trait` — only using the associated trait itself as a dyn bound is rejected. For example, `dyn Greetable` is fine even if `Greetable` has `trait Style;`, because the associated trait simply isn't used in the dyn context.

### What happens to trait and impl item definitions

For a **trait item** like `trait Elem: Clone;`, AST lowering converts the declaration bounds (`Clone`) into `TraitItemKind::Trait(GenericBounds)`. The `value` field (default, if any) is tracked via `has_value` but doesn't appear in the HIR trait item kind — it's stored separately for default-value lookup.

For an **impl item** like `trait Elem = Send + Clone;`, the value bounds are lowered to `ImplItemKind::Trait(GenericBounds)`. The declaration bounds are not carried here — they were already recorded on the trait definition and are checked later by `compare_impl_assoc_trait`.

This asymmetry mirrors how the data flows: declarations define constraints upward, values provide concrete bounds downward.

---

## Stage 3: Type Checking — Making Sure It All Adds Up

### What's a predicate?

At the type-checking level, all constraints are represented as **predicates** — formal statements that must be proven true. Common predicates include:

- `ClauseKind::Trait` — "type X implements trait Y" (e.g., `i32: Send`)
- `ClauseKind::Projection` — "associated type X::Y equals type Z"

Associated traits introduce a new predicate:

- `ClauseKind::AssocTraitBound` — "type B satisfies whatever the associated trait projection resolves to"

For example, `E: C::Elem` becomes:

```
AssocTraitBoundPredicate {
    self_ty: E,                    // the type being constrained
    projection: <C as Container>::Elem  // the associated trait to look up
}
```

### DefKind::AssocTrait

Every definition in the compiler has a `DefKind` — a tag identifying *what kind of thing* a given definition is. Functions are `DefKind::Fn`, structs are `DefKind::Struct`, associated types are `DefKind::AssocTy`, and so on. `DefKind` is used everywhere: in name resolution ("is this path referring to a type or a trait?"), in query dispatch ("which query handles this definition?"), and in error reporting ("expected type, found trait").

Associated traits get `DefKind::AssocTrait` — distinct from `DefKind::AssocTy`. This separation is critical because:

- **Name resolution** uses it to detect whether `C::Elem` in bound position refers to an associated trait (produce `GenericBound::AssocTraitBound`) or an associated type (produce a projection type).
- **Type queries** like `type_of()` must never be called on `AssocTrait` items — they have no type. If any code path tries, it hits `span_bug!` (a compiler panic).
- **Predicate construction** emits `ClauseKind::AssocTraitBound` for associated traits, not `ClauseKind::Projection` (which is for types). The `DefKind` is what drives this distinction.
- **Well-formedness checks**, **dyn compatibility**, and **method probing** all branch on `DefKind` to handle associated traits correctly.

In short, `DefKind` is the "dispatch tag" that lets every compiler pass know it's dealing with a constraint-carrying item rather than a type-carrying item.

### Checking impl values against declaration bounds

When you write:

```rust
trait Processor {
    trait Constraint: Clone;  // declaration bound
}

impl Processor for Bad {
    trait Constraint = Send;  // value
}
```

The function `compare_impl_assoc_trait` checks that the impl's value satisfies all declaration bounds:

1. It reads the declaration bounds from the trait definition (`Clone`)
2. It reads the value traits from the impl (`Send`)
3. For each declaration bound, it checks whether at least one value trait has it in its supertrait chain
4. Since `Send`'s supertraits do not include `Clone`, it emits:

```
error: associated trait bound `Constraint` is not satisfied:
       `Send` is not a subtrait of `Clone`
```

### Collecting item bounds

Both the declaration bounds (in the trait body) and the value bounds (in the impl body) are stored via the query `explicit_item_bounds()`. This is the same query system used for associated types' bounds. The type checker calls it to find out "what did this impl say the associated trait's value is?"

---

## Stage 4: The Trait Solver — Does This Type Actually Satisfy That Bound?

This is where the rubber meets the road. When the compiler sees:

```rust
process(SyncVec, 42i32);
```

It needs to prove: `i32: <SyncVec as Container>::Elem`. Since `SyncVec`'s impl says `trait Elem = Send`, this should reduce to proving `i32: Send`.

### How both trait solvers handle associated traits

The Rust compiler has two trait solvers: the **old solver** (the current default) and the **new solver** (`-Znext-solver`, under development). Both support associated traits, but through different code paths.

#### Old solver: fulfillment + evaluation

The old solver has two separate mechanisms:

- **Fulfillment** (obligation processing): This is the primary path for proving obligations. When the fulfillment engine encounters an `AssocTraitBound` predicate, it selects the impl for the trait reference, finds the associated trait item's value, extracts the concrete trait bounds, and emits them as new obligations. For example, given `i32: <SyncVec as Container>::Elem`, it finds `impl Container for SyncVec { trait Elem = Send; }`, extracts `Send`, and emits `i32: Send`.

- **Evaluation** (speculative checking): This path is used for diagnostics, coherence, and candidate filtering. It mirrors the fulfillment logic — selecting the impl, extracting value bounds, and recursively evaluating the resulting trait predicates.

#### New solver: `compute_assoc_trait_bound_goal`

The new solver has a dedicated function that handles `AssocTraitBound` goals:

#### Case 1: The type is still abstract (generic)

```rust
fn process<C: Container, E: C::Elem>(_c: C, _e: E) {}
```

Here, `C` is a type parameter — the solver doesn't know which concrete type it is. It can't look up an impl. So it just adds the *parent trait obligation* (`C: Container`) and moves on. The actual value-level checking happens later, either when `C` becomes concrete (monomorphization) or at the impl site (via `compare_impl_assoc_trait`).

Think of it like a deferred check: "I don't know *what* `C::Elem` is yet, but I trust that whatever implements `Container` will provide a valid `Elem`, and the impl-site check ensures that."

#### Case 2: The type is concrete

```rust
process(SyncVec, 42i32);
// The solver now knows: C = SyncVec, E = i32
```

Now the solver can resolve concretely:

1. **Find the impl**: Look up `impl Container for SyncVec`
2. **Fetch the associated trait item**: Find `trait Elem = Send` in that impl
3. **Read the value bounds**: `Elem`'s item bounds give us `Send`
4. **Emit new goals**: Replace the `AssocTraitBound` with `i32: Send`
5. **Prove**: `i32: Send` is trivially true. Done.

If instead we had `Rc<i32>` (which is `!Send`):

```rust
process(SyncVec, Rc::new(42)); // ERROR: Rc<i32>: Send is false
```

The solver would emit `Rc<i32>: Send`, which fails.

### Diagram: resolution flow

```
Goal:  i32: <SyncVec as Container>::Elem
                    │
                    ▼
        Is SyncVec a concrete type? YES
                    │
                    ▼
        Find: impl Container for SyncVec
                    │
                    ▼
        Fetch: trait Elem = Send
                    │
                    ▼
        Emit new goal: i32: Send
                    │
                    ▼
               i32: Send ✓  →  SUCCESS
```

---

## What Can't You Do (Yet)

### Associated traits in type position

```rust
let x: T::Elem = ...; // ERROR: Elem is a constraint, not a type
```

Associated traits produce *bounds*, not *types*. You can't use them where a type is expected.

### `dyn` with associated traits as bounds

```rust
fn with_dyn<T: Foo>(_x: &dyn T::Bar) {}
// ERROR: "associated traits cannot be used with `dyn`"
```

You cannot use an associated trait directly as a dyn bound. However, a trait that *has* associated traits **can** still be used as `dyn Trait`:

```rust
trait Greetable {
    trait Style;
    fn greet(&self) -> &str;
}

// This is fine — the associated trait simply isn't used in the dyn context.
fn greet_any(g: &dyn Greetable) -> &str {
    g.greet()
}
```

The restriction is specifically about using the associated trait *value* in a dyn bound position (like `dyn T::Bar`), not about the parent trait being dyn-compatible. This is analogous to how `dyn Iterator` works even though `Iterator` has an associated type — you just need to specify it (`dyn Iterator<Item = i32>`) when you use it. A future design could add similar binding syntax for associated traits (e.g., `dyn Container<Elem = Send>`).

### Bound expansion in impl method bodies

Associated trait bounds are fully expanded inside impl method bodies. When `Self::Arg` is known to be `IntoIterator<Item = i32>` inside the impl, you can use `IntoIterator` methods directly:

```rust
trait Handler {
    trait Arg;
    fn handle<T: Self::Arg>(&self, arg: T) -> i32;
}

impl Handler for SumHandler {
    trait Arg = IntoIterator<Item = i32>;
    fn handle<T: Self::Arg>(&self, arg: T) -> i32 {
        let mut sum = 0i32;
        for x in arg {   // Works — compiler knows T: IntoIterator<Item = i32>
            sum += x;
        }
        sum
    }
}
```

The compiler achieves this by expanding `AssocTraitBound` predicates in the method's parameter environment: when the impl is known, each `T: Self::Arg` predicate is supplemented with concrete predicates like `T: IntoIterator` and `<T as IntoIterator>::Item = i32`.

### Inherent impls

```rust
impl MyStruct {
    trait Foo = Send; // ERROR: only in trait impls
}
```

Associated traits only make sense inside `trait` and `impl Trait for Type` blocks.

---

## RFC Concepts Explained

This section walks through the key concepts from the RFC, with emphasis on what they mean practically.

### Associated traits vs. associated types

This is the central analogy. Just as associated types let a trait say "the implementor chooses *which type*," associated traits let a trait say "the implementor chooses *which constraints*."

| | Associated Type | Associated Trait |
|---|---|---|
| **Declaration** | `type Foo;` | `trait Foo;` |
| **Value** | `type Foo = i32;` | `trait Foo = Send;` |
| **Usage position** | Type position (`let x: T::Foo`) | Bound position (`E: T::Foo`) |
| **What it produces** | A type | A constraint (one or more trait bounds) |
| **Projection** | Yes — `T::Foo` is a type | No — `T::Foo` is not a type |

### Declaration bounds (supertraits on associated traits)

Just like a trait can have a supertrait (`trait Foo: Bar`), an associated trait declaration can require that every impl's value must include certain traits:

```rust
trait Processor {
    trait Constraint: Clone;  // "Whatever you set Constraint to, it must include Clone"
}

impl Processor for A {
    trait Constraint = Clone + Send;  // OK: supertraits of Clone include Clone ✓
}

impl Processor for B {
    trait Constraint = Send;  // ERROR: Send's supertraits don't include Clone
}
```

The compiler checks this via `compare_impl_assoc_trait`: for each declaration bound, it walks the supertrait hierarchy of each value trait to confirm it's present.

### Defaults

Like associated types, associated traits can have defaults:

```rust
trait Logger {
    trait Filter = Send;  // default
}

struct FileLogger;
impl Logger for FileLogger {}  // uses the default: Filter = Send

struct NetLogger;
impl Logger for NetLogger {
    trait Filter = Send + Sync;  // override
}
```

When an impl doesn't provide a value, the trait's default is used. The compiler stores this in `explicit_item_bounds()` on the trait item itself.

### Generic associated traits

Associated traits can have their own generic parameters, analogous to GATs (Generic Associated Types):

```rust
trait Transform {
    trait Constraint<T: Clone>;
}

impl Transform for Strict {
    trait Constraint<T: Clone> = PartialEq<T>;
}

fn compare<Tr: Transform, T: Clone + Tr::Constraint<T>>(a: T, b: T) -> bool {
    // T must satisfy whatever Constraint<T> resolves to
}
```

The parser handles generic parameters on the `AssocTraitItem`, and the type checker ensures they match between trait and impl (same number, same kinds).

### UFCS (Universal Function Call Syntax)

When a type implements multiple traits that each have an associated trait with the same name, you use UFCS to disambiguate:

```rust
trait Readable  { trait Constraint; }
trait Writable  { trait Constraint; }

// Without UFCS, E: T::Constraint is ambiguous
fn transfer<T: Readable + Writable, E: <T as Readable>::Constraint>(data: E) {}
//                                      ^^^^^^^^^^^^^^
//                                      "Constraint from Readable, not Writable"
```

In the HIR, the `constraint_trait` field carries the `DefId` of `Readable`, so the solver knows exactly which trait to resolve against.

### Multi-trait values

An associated trait's value can be a compound bound:

```rust
impl Container for SafeData {
    trait Elem = Send + Sync + Clone;  // three traits combined
}
```

Each of these (`Send`, `Sync`, `Clone`) becomes a separate `TraitPredicate` goal when the solver resolves `E: SafeData::Elem`.

### Trait inheritance

Associated traits are inherited through supertraits:

```rust
trait Base {
    trait Constraint;
}

trait Extended: Base {
    fn do_work(&self);
}

// Can use Base's associated trait through Extended
fn spawn<E: Extended, W: E::Constraint>(worker: E, _w: W) {}
```

### `impl Trait` integration

Associated traits work with `impl Trait` syntax:

```rust
fn make_element<C: Container>() -> impl C::Elem {
    42i32  // returns something that satisfies C::Elem
}
```

This creates an opaque type bounded by the associated trait. The caller sees `impl C::Elem`; the compiler checks that the returned value actually satisfies whatever `C::Elem` resolves to.

---

## Open Questions and Their Impact

These are unresolved points from the RFC that affect how the feature may evolve. Each has practical consequences for what code you can or can't write.

### 1. `where T::AssocTrait: OtherTrait` — constraints on associated trait values

**Status**: Not implemented; semantics unclear.

**What it means**: Can you write a bound that constrains what an associated trait's *value* must include?

```rust
fn foo<C: Container>()
where
    C::Elem: Debug  // "whatever C::Elem resolves to must include Debug"
{
    // ...
}
```

This is syntactically valid but currently semantically vacuous — `C::Elem` is a constraint, not a type, so there's no meaningful "type" to apply `Debug` to.

**Why it matters**: This would enable composing associated trait requirements at the call site, not just at the declaration site. Without it, you can only constrain values via declaration bounds (`trait Elem: Debug;`), which applies globally to all impls.

**Impact**: Medium. Declaration bounds cover many cases, but call-site restrictions on values would be more flexible.

### 2. Trait-level generic parameters

**Status**: Explicitly out of scope for this RFC.

**What it means**: The original issue (#2190) also proposed allowing traits themselves as generic parameters:

```rust
fn foo<Impl, trait Bound>() where Impl: Bound { ... }
```

This is a different and more general feature — it makes traits first-class generic parameters, not just associated items of another trait. The RFC deliberately defers this.

**Impact**: Low for associated traits specifically. This would be a separate feature with its own design space.

### 3. dyn binding syntax for associated traits

**Status**: Deferred; significant design work needed.

**What it means**: Currently, any trait with an associated trait is automatically dyn-incompatible — you can't use `dyn Container` if `Container` has `trait Elem;`. Making this work would require figuring out how to represent the associated trait's value in a vtable, which is a non-trivial design challenge.

**Impact**: Medium. Many use cases for associated traits (plugin systems, runtime abstraction) also use trait objects. The inability to combine both is a meaningful restriction.

### 6. Negative associated trait bounds

**Status**: Deferred; intersects with negative impls.

**What it means**: Could you write `trait Elem = !Send;`? This would express "this type must *not* implement Send." Rust's negative impl story is still evolving, so this is left for the future.

**Impact**: Low for most users. Negative bounds are niche.

---

## Summary: How It All Fits Together

```
                          YOUR CODE
                             │
     ┌───────────────────────┼──────────────────────────┐
     │                       │                          │
     ▼                       ▼                          ▼
  DECLARATION             IMPL VALUE                 USAGE
  trait Elem;             trait Elem = Send;          E: C::Elem
     │                       │                          │
     ▼                       ▼                          ▼
  AssocTraitItem          AssocTraitItem              AssocTraitBound
  (AST node)              (AST node)                 (HIR node)
     │                       │                          │
     ▼                       ▼                          ▼
  DefKind::AssocTrait     item_bounds() → [Send]     ClauseKind::AssocTraitBound
  in trait's assoc item   stored as predicates        {self_ty: E, projection: C::Elem}
     │                       │                          │
     └──────┬────────────────┘                          │
            ▼                                           ▼
  compare_impl_assoc_trait                    TRAIT SOLVER
  "does Send satisfy                          (old or new)
   declaration bounds?"                        "find impl for C,
                                               read Elem = Send,
                                               prove E: Send"
            │                                           │
            ▼                                           ▼
    ✓ or error                                  ✓ or error
```

The feature adds a new kind of associated item that carries *constraints* instead of *types* through the compiler. Each stage — parsing, lowering, type checking, trait solving — handles this new kind explicitly, with its own `DefKind`, its own `ClauseKind`, its own comparison logic, and its own solver goal. This clean separation is what makes the feature work correctly without disrupting the existing associated type machinery.
