#![feature(associated_traits)]

pub trait Container {
    trait Elem;
}

pub struct SyncVec;

impl Container for SyncVec {
    trait Elem = Send;
}
