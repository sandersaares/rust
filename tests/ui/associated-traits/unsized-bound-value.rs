//@ check-pass
// Test that ?Sized works as an associated trait value,
// as well as combinations like Send + ?Sized.
// See RFC #2190 comment by AndreiCravtov.

#![feature(associated_traits)]

trait Container {
    trait Bounds;
}

// ?Sized means "no Sized requirement" — the most permissive bound
struct FlexibleContainer;
impl Container for FlexibleContainer {
    trait Bounds = ?Sized;
}

// Send + ?Sized
struct SendFlexibleContainer;
impl Container for SendFlexibleContainer {
    trait Bounds = Send + ?Sized;
}

// Just Send (default Sized still applies at use site)
struct StrictContainer;
impl Container for StrictContainer {
    trait Bounds = Send;
}

// Using with sized types always works
fn use_container_sized<C: Container, T: C::Bounds>(_t: T) {}

// Using with unsized types requires + ?Sized at the use site
fn use_container_unsized<C: Container, T: C::Bounds + ?Sized>(_t: &T) {}

fn main() {
    // Sized types work everywhere
    use_container_sized::<FlexibleContainer, i32>(42);
    use_container_sized::<SendFlexibleContainer, i32>(42);
    use_container_sized::<StrictContainer, i32>(42);

    // Unsized types work when use-site adds ?Sized
    use_container_unsized::<FlexibleContainer, str>("hello");
    use_container_unsized::<FlexibleContainer, [u8]>(&[1, 2, 3]);
    use_container_unsized::<SendFlexibleContainer, [u8]>(&[1, 2, 3]);
}
