#![deny(missing_docs)]

//! Lets you open and talk to a single cross-platform web view.
//!
//! The webpage also has some extra capabilities:
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

mod handler;
mod options;
mod platform;

pub use handler::*;
pub use options::*;

static LOADED: AtomicBool = ATOMIC_BOOL_INIT;

/// A link between the web view and your application.
#[derive(Clone, Copy)]
pub struct Tether(PhantomData<*mut ()>);

impl Tether {
    /// Creates a new window builder.
    ///
    /// This is usually one of the first functions you call in your application.
    pub fn builder() -> Options<'static, ()> {
        Options::new()
    }

    /// Evaluates some JavaScript in the current webpage.
    pub fn eval(self, js: &str) {
        platform::eval(js)
    }

    /// Sets the current webpage to the provided HTML string.
    pub fn load(self, html: &str) {
        platform::load(html)
    }

    /// Schedules the provided function to eventually run on the UI thread.
    ///
    /// This is the only way to change the user interface from another thread.
    pub fn dispatch<F: FnOnce(Tether) + Send + 'static>(f: F) {
        if LOADED.load(SeqCst) {
            platform::dispatch(f)
        }
    }

    /// Opens the window with the provided options.
    pub fn start<H: Handler>(opts: Options<H>) -> ! {
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
