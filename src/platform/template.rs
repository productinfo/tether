use {Handler, Options, Tether};

pub fn start<H: Handler>(_: Options<H>) {
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
