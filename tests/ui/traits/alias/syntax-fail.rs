#![feature(trait_alias)]

trait Foo {}
auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`
unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`

trait C: Ord = Eq; //~ ERROR associated traits are experimental
trait D: = Eq;

fn main() {}
