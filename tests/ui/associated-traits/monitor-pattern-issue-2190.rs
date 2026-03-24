//@ check-pass
// Kiiyya's Monitor pattern from rust-lang/rfcs#2190.
// Associated trait constrains what kind of monitors are acceptable.

#![feature(associated_traits)]
#![allow(incomplete_features)]

trait Monitor {
    type Action;
}

trait SetLikeMonitor: Monitor {
    fn mk_put() -> Self::Action;
    fn mk_rm() -> Self::Action;
}

trait MyTrait {
    trait Mon: Monitor;
}

struct SillySet;

struct MyMonitor;

impl Monitor for MyMonitor {
    type Action = String;
}

impl SetLikeMonitor for MyMonitor {
    fn mk_put() -> String { "put".into() }
    fn mk_rm() -> String { "rm".into() }
}

impl MyTrait for SillySet {
    // Constraining Mon to SetLikeMonitor via supertrait bound
    trait Mon = SetLikeMonitor<Action = String>;
}

// Using the associated trait as a constraint
fn use_trait<T: MyTrait, M: T::Mon>(_t: &T, _m: &M) {}

// With extra bound at call site to use methods
fn use_trait_methods<T: MyTrait, M: T::Mon + SetLikeMonitor>(_t: &T, _m: &M) {
    let _ = M::mk_put();
}

fn main() {
    let s = SillySet;
    let m = MyMonitor;
    use_trait(&s, &m);
    use_trait_methods(&s, &m);
}
