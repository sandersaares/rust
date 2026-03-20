struct S<T = u8>(T);
trait Tr<T = u8> {}

impl Self for S {} //~ ERROR expected trait, found self type `Self`
impl Self::N for S {} // accepted as associated trait bound (no error)

fn main() {}
