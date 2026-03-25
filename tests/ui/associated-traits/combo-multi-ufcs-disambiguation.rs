//@ check-pass
// Complex combination: multiple associated traits with UFCS disambiguation
// in a realistic scenario — a type implements two "capability" traits that
// both define associated traits with the same name.

#![feature(associated_traits)]

trait Readable {
    trait Constraint;
}

trait Writable {
    trait Constraint;
}

struct FileHandle;
impl Readable for FileHandle {
    trait Constraint = Send;
}
impl Writable for FileHandle {
    trait Constraint = Send + Sync;
}

// Without UFCS, this would be ambiguous since both traits have "Constraint"
fn read_data<T: Readable, D: <T as Readable>::Constraint>(_handle: &T, _data: D) {}
fn write_data<T: Writable, D: <T as Writable>::Constraint>(_handle: &T, _data: D) {}

// Both at once
fn read_write<T: Readable + Writable>(
    _handle: &T,
    _rdata: impl <T as Readable>::Constraint,
    _wdata: impl <T as Writable>::Constraint,
) {}

fn main() {
    let fh = FileHandle;
    read_data(&fh, 42i32);
    write_data(&fh, 42i32);
    read_write(&fh, 42i32, 42i32);
}
