use Tether;

/// An event handler.
pub trait Handler: 'static {
    /// Called when a message has been sent from the webpage.
    fn message(&mut self, win: Tether, msg: &str) {
        let _ = (win, msg);
    }

    /// A warning that the system *may* be about to unceremoniously kill
    /// your app. Saving things is probably a good idea.
    fn suspend(&mut self, win: Tether) {
        let _ = win;
    }
}

impl<F: FnMut(Tether, &str) + 'static> Handler for F {
    fn message(&mut self, win: Tether, msg: &str) {
        self(win, msg)
    }
}

impl Handler for () {}
