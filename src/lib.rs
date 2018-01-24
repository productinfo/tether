#[macro_use]
extern crate cfg_if;

use std::marker::PhantomData;
use std::sync::atomic::{ATOMIC_BOOL_INIT, AtomicBool};
use std::sync::atomic::Ordering::SeqCst;

mod handler;
mod options;
mod platform;

pub use handler::*;
pub use options::*;

//TODO: Use !impl instead of PhantomData.

static LOADED: AtomicBool = ATOMIC_BOOL_INIT;

#[derive(Clone, Copy)]
pub struct Tether(PhantomData<*mut ()>);

impl Tether {
    pub fn builder() -> Options<'static, ()> {
        Options::new()
    }

    pub fn eval(self, js: &str) {
        platform::eval(js)
    }

    pub fn load(self, html: &str) {
        platform::load(html)
    }

    pub fn dispatch<F: FnOnce(Tether) + Send + 'static>(f: F) {
        platform::dispatch(f)
    }

    pub fn start<H: Handler>(opts: Options<H>) {
        if !LOADED.swap(true, SeqCst) {
            platform::start(opts)
        } else {
            panic!("Tried to start Tether more than once!")
        }
    }

    unsafe fn new() -> Tether {
        Tether(PhantomData)
    }
}
