#![deny(missing_docs)]

//! Lets you open and talk to a single cross-platform web view.
//!
//! The webpage also has some extra capabilities:
//!
//! - `window.tether(string)` sends a message to your application.
//! - `document.title` sets the window's title.

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        // No runtime dependencies.
    } else if #[cfg(target_os = "macos")] {
        extern crate cocoa;
        extern crate dispatch;
        #[macro_use]
        extern crate objc;
    } else {
        extern crate glib;
        extern crate gtk;
        extern crate webkit2gtk;
    }
}

use std::marker::PhantomData;
use std::sync::atomic::{ATOMIC_BOOL_INIT, AtomicBool};
use std::sync::atomic::Ordering::SeqCst;

mod escape;
mod handler;
mod options;
mod platform;

pub use escape::escape;
pub use handler::Handler;
pub use options::Options;

static LOADED: AtomicBool = ATOMIC_BOOL_INIT;

/// Creates a new window builder. You should probably call this first!
pub fn builder() -> Options<'static, ()> {
    Options::new()
}

/// Schedules the provided function to eventually run on the UI thread.
///
/// This is the only way to talk to the window from another thread.
pub fn dispatch<F: FnOnce(Window) + Send + 'static>(f: F) {
    if LOADED.load(SeqCst) {
        platform::dispatch(f)
    }
}

fn start<H: Handler>(opts: Options<H>) -> ! {
    if !LOADED.swap(true, SeqCst) {
        platform::start(opts)
    } else {
        panic!("Tried to initialize the window more than once!")
    }
}

/// A link between the web view and your application.
#[derive(Clone, Copy)]
pub struct Window(PhantomData<*mut ()>);

impl Window {
    /// Evaluates some JavaScript in the current webpage.
    pub fn eval(self, js: &str) {
        platform::eval(js)
    }

    /// Sets the current webpage to the provided HTML string.
    pub fn load(self, html: &str) {
        platform::load(html)
    }

    unsafe fn new() -> Window {
        Window(PhantomData)
    }
}
