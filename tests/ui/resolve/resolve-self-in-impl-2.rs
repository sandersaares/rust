struct S<T = u8>(T);
trait Tr<T = u8> {}

impl Self for S {} //~ ERROR expected trait, found self type `Self`
impl Self::N for S {} // Accepted as associated trait bound on Self

fn main() {}
