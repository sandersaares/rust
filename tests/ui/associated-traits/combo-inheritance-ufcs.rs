//@ check-pass
// Complex combination: trait inheritance with associated traits,
// multiple levels of trait hierarchy.

#![feature(associated_traits)]

trait Base {
    trait Requirement;
}

trait Extended: Base {
    trait ExtraRequirement;
}

struct Impl;
impl Base for Impl {
    trait Requirement = Send;
}

impl Extended for Impl {
    trait ExtraRequirement = Sync;
}

// Using both inherited and own associated traits from the outside
fn use_extended<E: Extended, T: E::Requirement + E::ExtraRequirement>(_e: &E, _t: T) {}

// UFCS disambiguating from Extended (which inherits Requirement from Base)
fn ufcs_inherited<E: Extended, T: <E as Base>::Requirement>(_e: &E, _t: T) {}

// Both at once with UFCS
fn ufcs_both<E: Extended, T: <E as Base>::Requirement + <E as Extended>::ExtraRequirement>(
    _e: &E,
    _t: T,
) {}

fn main() {
    let imp = Impl;
    // i32: Send + Sync ✓
    use_extended(&imp, 42i32);
    ufcs_inherited(&imp, 42i32);
    ufcs_both(&imp, 42i32);
}
