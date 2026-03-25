//@ check-pass
// Test UFCS (Universal Function Call Syntax) for associated traits.
// Syntax: `<T as Trait>::AssocTrait` in bound position.
// See RFC #2190 — this enables disambiguation when multiple traits
// have same-named associated traits.

#![feature(associated_traits)]

trait Container {
    trait Elem;
}

trait OtherContainer {
    trait Elem;
}

struct MyContainer;
impl Container for MyContainer {
    trait Elem = Send;
}

impl OtherContainer for MyContainer {
    trait Elem = Clone;
}

// UFCS syntax specifies which trait to resolve the associated trait from
fn use_container_elem<T: Container, E: <T as Container>::Elem>(_e: E) {}
fn use_other_elem<T: OtherContainer, E: <T as OtherContainer>::Elem>(_e: E) {}

// UFCS in where clause (single trait)
fn where_ufcs<T: Container, E>(_e: E)
where
    E: <T as Container>::Elem,
{}

// UFCS for disambiguation: T implements both traits with same-named assoc trait
fn disambiguate<T: Container + OtherContainer, E>(_e: E)
where
    E: <T as Container>::Elem,
{}

fn disambiguate_other<T: Container + OtherContainer, E>(_e: E)
where
    E: <T as OtherContainer>::Elem,
{}

fn main() {
    use_container_elem::<MyContainer, i32>(42);
    use_other_elem::<MyContainer, String>(String::from("hello"));
    where_ufcs::<MyContainer, i32>(42);
    disambiguate::<MyContainer, i32>(42);
    disambiguate_other::<MyContainer, String>(String::from("hello"));
}
