#![feature(associated_traits)]
#![allow(incomplete_features)]

pub trait Container {
    trait Elem;
}

pub struct SyncVec;

impl Container for SyncVec {
    trait Elem = Send;
}
