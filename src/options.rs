use {start, Handler};

/// Provides some configuration options for the window.
pub struct Options<'a, H> {
    pub(crate) html: &'a str,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) fullscreen: bool,
    pub(crate) handler: H,
    pub(crate) minimum_size: Option<(usize, usize)>,
}

impl Options<'static, ()> {
    /// Creates a new configuration with sensible defaults.
    ///
    /// - The HTML is an empty string.
    /// - The initial window size is 640x480.
    /// - The minimum window size is the inital window size.
    /// - The window is not initially in fullscreen mode.
    pub fn new() -> Self {
        Self {
            html: "",
            width: 640,
            height: 480,
            fullscreen: false,
            handler: (),
            minimum_size: None,
        }
    }
}

impl<'a, H> Options<'a, H> {
    /// Sets the initially displayed HTML.
    pub fn html<'aa>(self, html: &'aa str) -> Options<'aa, H> {
        let Options { width, height, fullscreen, handler, minimum_size, .. } = self;
        Options { html, width, height, fullscreen, handler, minimum_size }
    }

    /// Sets the preferred size.
    pub fn size(self, width: usize, height: usize) -> Options<'a, H> {
        let Options { html, fullscreen, handler, minimum_size, .. } = self;
        Options { html, width, height, fullscreen, handler, minimum_size }
    }

    /// Sets the preferred minimum size.
    pub fn minimum_size(self, width: usize, height: usize) -> Options<'a, H> {
        let minimum_size = Some((width, height));
        let Options { html, width, height, fullscreen, handler, .. } = self;
        Options { html, width, height, fullscreen, handler, minimum_size }
    }

    /// Sets whether the window should initially be in fullscreen mode.
    pub fn fullscreen(self, fullscreen: bool) -> Options<'a, H> {
        let Options { html, width, height, handler, minimum_size, .. } = self;
        Options { html, width, height, fullscreen, handler, minimum_size }
    }

    /// Sets the event handler.
    pub fn handler<HH>(self, handler: HH) -> Options<'a, HH> {
        let Options { html, width, height, fullscreen, minimum_size, .. } = self;
        Options { html, width, height, fullscreen, handler, minimum_size }
    }
}

impl<'a, H: Handler> Options<'a, H> {
    /// Opens the window.
    pub fn start(self) -> ! {
        start(self)
    }
}
