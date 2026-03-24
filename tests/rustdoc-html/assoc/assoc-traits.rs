#![crate_type="lib"]
#![feature(associated_traits)]
#![allow(incomplete_features)]

//@ has assoc_traits/trait.Container.html

pub trait Container {
    //@ has - '//*[@id="associatedtype.ElementConstraint"]//h4[@class="code-header"]' 'trait ElementConstraint'
    trait ElementConstraint;
}

pub struct SyncContainer;

//@ has assoc_traits/struct.SyncContainer.html
impl Container for SyncContainer {
    //@ has - '//*[@id="associatedtype.ElementConstraint"]//h4[@class="code-header"]' 'trait ElementConstraint'
    trait ElementConstraint = Send;
}
