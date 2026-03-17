# Copilot Instructions for rust-lang/rust

This is the main repository for the Rust programming language compiler (`rustc`),
standard library, and related tooling.

## Build System

The build system is a multi-stage bootstrap. Use `./x` (or `./x.py`) as the entry point.
Run `./x setup` for first-time configuration (creates `bootstrap.toml`).

### Key Commands

```bash
./x check                    # Fast type-check (no codegen), ~30s
./x build                    # Build stage 1 compiler + stdlib
./x build --keep-stage 0     # Fastest rebuild (reuses stage 0)
./x fmt                      # Format code (rustfmt)
./x fmt --check              # Check formatting without modifying
./x test tidy                # Run tidy lints (style, deps, features)
./x clippy compiler/rustc_middle  # Clippy a specific crate
```

### Testing

```bash
./x test tests/ui                              # Run all UI tests
./x test tests/ui/borrowck/access-mode-in-closures.rs  # Run a single test
./x test tests/ui --bless                      # Update expected outputs
./x test tests/mir-opt --bless                 # Update MIR dump expectations
./x test tests/codegen-llvm                    # Run codegen tests (FileCheck)
./x test tests/run-make                        # Integration tests
```

For fastest iteration on compiler changes: `./x build --keep-stage 0` then `./x test tests/ui --stage 1`.

## Architecture

### Compilation Pipeline

```
Source → Parse (rustc_parse) → AST (rustc_ast)
  → Expand macros (rustc_expand) → Name resolution (rustc_resolve)
  → Lower to HIR (rustc_ast_lowering) → HIR (rustc_hir)
  → Type checking (rustc_hir_analysis, rustc_hir_typeck)
  → Trait solving (rustc_trait_selection)
  → Lower to THIR → MIR (rustc_mir_build)
  → Borrow checking (rustc_borrowck)
  → MIR optimizations (rustc_mir_transform)
  → Codegen (rustc_codegen_ssa → rustc_codegen_llvm)
  → LLVM IR → Machine code → Linking
```

### Key Crates

- **`rustc_middle`** — Central crate: defines `TyCtxt`, `Ty`, MIR, and the query system
- **`rustc_interface`** — Compiler entry point; `passes.rs` orchestrates the pipeline
- **`rustc_hir`** — High-level IR (after parsing/lowering, before type checking)
- **`rustc_hir_typeck`** — Function body type inference and checking
- **`rustc_trait_selection`** — Trait obligation solving
- **`rustc_borrowck`** — Borrow checker (MIR-based)
- **`rustc_mir_transform`** — MIR optimization passes
- **`rustc_codegen_ssa`** — Backend-agnostic codegen layer
- **`rustc_errors`** — Diagnostic infrastructure
- **`rustc_span`** — Source locations, symbols, hygiene

### Query System

All compiler analysis is organized as lazy, memoized **queries** on `TyCtxt<'tcx>`.
Queries enable incremental compilation—only recompute what changed. Access queries
via methods on `tcx` (e.g., `tcx.type_of(def_id)`, `tcx.typeck(def_id)`).

### Standard Library Layering

`core` (no dependencies) → `alloc` (heap allocation) → `std` (I/O, threads, OS).

### Bootstrap Stages

- **Stage 0**: Pre-built compiler downloaded from CI
- **Stage 1**: Compiler built by stage 0 (used for development/testing)
- **Stage 2**: Compiler built by stage 1 (used for releases)

## Conventions

### `TyCtxt<'tcx>`

The central compiler context, passed as `tcx` everywhere. Holds query caches,
interned types, and session state. All types (`Ty<'tcx>`) are arena-allocated
and interned (deduplicated) through it.

### `hir::Ty` vs `ty::Ty`

- `hir::Ty` — Syntactic type as written in source (used during lowering/early analysis)
- `ty::Ty<'tcx>` — Semantic, resolved type after type-checking (used through MIR/codegen)

### Diagnostics

Prefer `#[derive(Diagnostic)]` and `#[derive(Subdiagnostic)]` for structured errors:

```rust
#[derive(Diagnostic)]
#[diag(hir_analysis_field_already_declared, code = E0124)]
pub struct FieldAlreadyDeclared {
    pub field_name: Ident,
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(hir_analysis_previous_decl_label)]
    pub prev_span: Span,
}
```

Diagnostic messages use Fluent (`.ftl` files) for localization. Each compiler crate
has its own `messages.ftl`.

### ICE (Internal Compiler Error) Macros

Use `bug!()` / `span_bug!()` for unreachable states (not `panic!` or `unreachable!`):

```rust
span_bug!(span, "unexpected type: {:?}", ty);
bug!("impossible case reached");
```

Use `tcx.dcx().span_delayed_bug(span, "msg")` when the error should only fire if
no other errors were emitted.

### Error Codes

Error codes (`E0XXX`) are defined in `compiler/rustc_error_codes/`. Each needs:
- A `.md` doc file in `compiler/rustc_error_codes/src/error_codes/`
- A UI test in `tests/ui/error-codes/`

Never remove error codes; mark them as "no longer emitted" if obsolete.

### Feature Gates

Defined in `compiler/rustc_feature/src/`. Use `unstable`, `accepted`, or `removed`
status. Never delete a feature gate—move it to the appropriate status module.

### Test Directives

Test files use `//@ ` prefixed directives:

```rust
//@ compile-flags: -O
//@ edition: 2021
//@ only-x86_64
//@ ignore-windows
//@ revisions: opt noopt
//@ [opt] compile-flags: -O
//@ run-pass
//@ should-fail
//@ needs-llvm-components: x86
//@ aux-build: helper.rs
```

Expected errors are annotated inline:
```rust
let x: i32 = "hello";
//~^ ERROR mismatched types
```

### Formatting

Configured in `rustfmt.toml`: style edition 2024, `imports_granularity = "Module"`,
`group_imports = "StdExternalCrate"`, `merge_derives = false`. Most test directories
are excluded from formatting. Run `./x fmt` before committing.

### Tidy

The `tidy` tool (`src/tools/tidy/`) enforces repo-wide checks: line length (100 chars
for Rust, 80 for error code docs), no tabs, feature gate validation, dependency rules,
and alphabetical ordering in marked sections (`// tidy-alphabetical-start`).
Opt out per-line with `// ignore-tidy-linelength` etc.
