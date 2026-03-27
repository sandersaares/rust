// Test: HttpClient<Isolated> is !Send because Rc<RefCell<State>> is !Send.
// This is the negative counterpart to the universe pattern test.

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

trait RefLike<T>: Clone { fn new(val: T) -> Self; }
trait CellLike<T> { fn new(val: T) -> Self; }
impl<T> RefLike<T> for Arc<T> { fn new(val: T) -> Self { Arc::new(val) } }
impl<T> RefLike<T> for Rc<T> { fn new(val: T) -> Self { Rc::new(val) } }
impl<T> CellLike<T> for Mutex<T> { fn new(val: T) -> Self { Mutex::new(val) } }
impl<T> CellLike<T> for RefCell<T> { fn new(val: T) -> Self { RefCell::new(val) } }

trait Universe {
    trait BoundsIn;
    trait BoundsOut;
    type Ref<T: Self::BoundsOut>: RefLike<T> + Self::BoundsOut;
    type Cell<T: Self::BoundsIn>: CellLike<T> + Self::BoundsOut;
}

struct Isolated;
impl Universe for Isolated {
    trait BoundsIn = 'static;
    trait BoundsOut = 'static;
    type Ref<T: Self::BoundsOut> = Rc<T>;
    type Cell<T: Self::BoundsIn> = RefCell<T>;
}

struct State { x: u64 }

struct HttpClient<U: Universe> {
    state: U::Ref<U::Cell<State>>,
}

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<HttpClient<Isolated>>();
    //~^ ERROR `Rc<RefCell<State>>` cannot be sent between threads safely
}
