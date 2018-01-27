//! The template for the platform-specific implementation.
//!
//! Guarantees:
//! - `start` will be called at most once.
//! - `load` and `eval` will only be called on the UI thread.
//! - None of the functions will be called before `start`.
//!
//! Javascript interface:
//! - `window.tether(string)` should send the message to the handler.
//! - Setting the document's title should change the window's title.
//! - `open` and `close` are currently undefined.
//!
//! Freedoms:
//! - You may kill the application once the window has been closed.

use {Handler, Options, Tether};

pub fn start<H: Handler>(_: Options<H>) -> ! {
    unimplemented!()
}

pub fn load(_: &str) {
    unimplemented!()
}

pub fn eval(_: &str) {
    unimplemented!()
}

pub fn dispatch<F: FnOnce(Tether) + Send + 'static>(f: F) {
    unimplemented!()
}
