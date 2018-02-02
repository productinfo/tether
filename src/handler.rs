use Window;

/// An event handler.
pub trait Handler: 'static {
    /// Called when a message has been sent from the webpage.
    fn message(&mut self, win: Window, msg: &str) {
        let _ = (win, msg);
    }

    /// A warning that the system *may* be about to unceremoniously kill
    /// your app. Saving things is probably a good idea.
    fn suspend(&mut self, win: Window) {
        let _ = win;
    }
}

impl<F: FnMut(Window, &str) + 'static> Handler for F {
    fn message(&mut self, win: Window, msg: &str) {
        self(win, msg)
    }
}

impl Handler for () {}
