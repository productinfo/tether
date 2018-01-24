use Tether;

pub trait Handler: 'static {
    fn message(&mut self, Tether, &str) {}
    fn close(&mut self) {}
}

impl<F: FnMut(Tether, &str) + 'static> Handler for F {
    fn message(&mut self, win: Tether, msg: &str) {
        self(win, msg)
    }
}

impl Handler for () {}
