use {Handler, Options, Tether};
use std::{process, slice, str};
use std::os::raw::{c_int, c_void};

pub fn start<H: Handler>(opts: Options<H>) -> ! {
    unsafe {
        tether_start(
            TetherString::new(opts.html),
            opts.width,
            opts.height,
            if opts.fullscreen { 1 } else { 0 },

            Box::into_raw(Box::new(opts.handler)) as *mut c_void,
            message::<H>,
            suspend::<H>,
            release::<H>,
        )
    };

    process::exit(0)
}

unsafe extern "C" fn message<H: Handler>(handler: *mut c_void, msg: TetherString) {
    let handler = handler as *mut H;
    (*handler).message(Tether::new(), msg.to_str());
}

unsafe extern "C" fn suspend<H: Handler>(handler: *mut c_void) {
    let handler = handler as *mut H;
    (*handler).suspend(Tether::new());
}

unsafe extern "C" fn release<H: Handler>(handler: *mut c_void) {
    let handler = handler as *mut H;
    Box::from_raw(handler);
}

pub fn load(html: &str) {
    unsafe {
        tether_load(TetherString::new(html))
    };
}

pub fn eval(js: &str) {
    unsafe {
        tether_eval(TetherString::new(js))
    };
}

pub fn dispatch<F: FnOnce(Tether) + Send + 'static>(f: F) {
    unsafe {
        tether_dispatch(
            Box::into_raw(Box::new(f)) as *mut c_void,
            exec::<F>,
        )
    };

    unsafe extern "C" fn exec<F: FnOnce(Tether) + Send + 'static>(f: *mut c_void) {
        Box::from_raw(f as *mut F)(Tether::new());
    }
}

// FOREIGN STUFF

#[repr(C)]
struct TetherString {
    len: usize,
    ptr: *const u8,
}

impl TetherString {
    fn new(x: &str) -> TetherString {
        TetherString { len: x.len(), ptr: x.as_ptr() }
    }

    unsafe fn to_str<'a>(self) -> &'a str {
        str::from_utf8_unchecked(slice::from_raw_parts(self.ptr, self.len))
    }
}

extern "C" {
    fn tether_start(
        html: TetherString,
        width: usize,
        height: usize,
        fullscreen: c_int,

        han_data: *mut c_void,
        han_message: unsafe extern "C" fn(*mut c_void, TetherString),
        han_suspend: unsafe extern "C" fn(*mut c_void),
        han_drop: unsafe extern "C" fn(*mut c_void),
    );

    fn tether_load(html: TetherString);

    fn tether_eval(js: TetherString);

    fn tether_dispatch(
        data: *mut c_void,
        exec: unsafe extern "C" fn(*mut c_void),
    );
}
