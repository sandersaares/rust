//@ check-pass
// Real-world pattern: Universe-parameterized types using multiple associated
// traits (BoundsIn/BoundsOut) to separate input requirements from output
// guarantees. Derived from the "missinglink" demo.
//
// Key patterns tested:
// - Multiple associated traits with different roles in one trait
// - GATs bounded by different associated traits (Ref by BoundsOut, Cell by BoundsIn)
// - Nested GAT usage: U::Ref<U::Cell<State>>
// - No where-clause leaking: downstream AppService needs no State bounds
// - Compile-time Send+Sync verification on concrete instantiations

#![feature(associated_traits)]
#![allow(incomplete_features)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

trait RefLike<T>: Clone {
    fn new(val: T) -> Self;
}
trait CellLike<T> {
    fn new(val: T) -> Self;
}
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

struct Shared;
impl Universe for Shared {
    trait BoundsIn = Send + 'static;
    trait BoundsOut = Send + Sync + 'static;
    type Ref<T: Self::BoundsOut> = Arc<T>;
    type Cell<T: Self::BoundsIn> = Mutex<T>;
}

struct Isolated;
impl Universe for Isolated {
    trait BoundsIn = 'static;
    trait BoundsOut = 'static;
    type Ref<T: Self::BoundsOut> = Rc<T>;
    type Cell<T: Self::BoundsIn> = RefCell<T>;
}

struct State { base_url: String }

// No `where State: U::BoundsIn` needed — GAT bounds checked at monomorphization
struct HttpClient<U: Universe> {
    state: U::Ref<U::Cell<State>>,
}

impl<U: Universe> HttpClient<U> {
    fn new(base_url: String) -> Self {
        HttpClient { state: RefLike::new(CellLike::new(State { base_url })) }
    }
    fn get(&self, _path: &str) -> String { String::from("200 OK") }
    fn clone_ref(&self) -> Self { HttpClient { state: self.state.clone() } }
}

// Downstream type — no where clause leaking
struct AppService<U: Universe> {
    client: HttpClient<U>,
}

impl<U: Universe> AppService<U> {
    fn new(base_url: String) -> Self {
        AppService { client: HttpClient::new(base_url) }
    }
    fn fetch(&self) -> String { self.client.get("/data") }
}

// Compile-time: Shared universe types are Send + Sync
const _: () = {
    fn assert_send_sync<T: Send + Sync>() {}
    fn check() {
        assert_send_sync::<HttpClient<Shared>>();
        assert_send_sync::<AppService<Shared>>();
    }
};

fn main() {
    let svc = AppService::<Shared>::new("https://example.com".into());
    assert_eq!(svc.fetch(), "200 OK");

    let svc = AppService::<Isolated>::new("https://example.com".into());
    assert_eq!(svc.fetch(), "200 OK");

    // clone_ref works for both universes
    let client = HttpClient::<Shared>::new("https://example.com".into());
    let _cloned = client.clone_ref();
}
