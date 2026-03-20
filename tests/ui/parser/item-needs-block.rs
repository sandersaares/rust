trait Trait;
//~^ ERROR associated traits are experimental

impl Trait for ();
//~^ ERROR expected `{}`, found `;`

enum Enum;
//~^ ERROR expected `{}`, found `;`

fn main() {}
