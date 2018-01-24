use {Handler, Tether};

pub struct Options<'a, H> {
    pub html: &'a str,
    pub width: usize,
    pub height: usize,
    pub fullscreen: bool,
    pub handler: H,
}

impl Options<'static, ()> {
    pub fn new() -> Self {
        Self {
            html: "",
            width: 640,
            height: 480,
            fullscreen: false,
            handler: (),
        }
    }
}

impl<'a, H> Options<'a, H> {
    pub fn html<'aa>(self, html: &'aa str) -> Options<'aa, H> {
        let Options { width, height, fullscreen, handler, .. } = self;
        Options { html, width, height, fullscreen, handler }
    }

    pub fn size(self, width: usize, height: usize) -> Options<'a, H> {
        let Options { html, fullscreen, handler, .. } = self;
        Options { html, width, height, fullscreen, handler }
    }

    pub fn fullscreen(self, fullscreen: bool) -> Options<'a, H> {
        let Options { html, width, height, handler, .. } = self;
        Options { html, width, height, fullscreen, handler }
    }

    pub fn handler<HH>(self, handler: HH) -> Options<'a, HH> {
        let Options { html, width, height, fullscreen, .. } = self;
        Options { html, width, height, fullscreen, handler }
    }
}

impl<'a, H: Handler> Options<'a, H> {
    pub fn start(self) {
        Tether::start(self)
    }
}
