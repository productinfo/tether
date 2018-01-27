use {Handler, Tether};

/// Provides some configuration options for the window.
pub struct Options<'a, H> {
    /// The initially displayed HTML.
    pub html: &'a str,
    /// The preferred width.
    pub width: usize,
    /// The preferred height.
    pub height: usize,
    /// Whether the window should initially be in fullscreen mode.
    pub fullscreen: bool,
    /// The event handler.
    pub handler: H,
}

impl Options<'static, ()> {
    /// Creates a new configuration with sensible defaults.
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
    /// Sets the initially displayed HTML.
    pub fn html<'aa>(self, html: &'aa str) -> Options<'aa, H> {
        let Options { width, height, fullscreen, handler, .. } = self;
        Options { html, width, height, fullscreen, handler }
    }

    /// Sets the preferred size.
    pub fn size(self, width: usize, height: usize) -> Options<'a, H> {
        let Options { html, fullscreen, handler, .. } = self;
        Options { html, width, height, fullscreen, handler }
    }

    /// Sets whether the window should initially be in fullscreen mode.
    pub fn fullscreen(self, fullscreen: bool) -> Options<'a, H> {
        let Options { html, width, height, handler, .. } = self;
        Options { html, width, height, fullscreen, handler }
    }

    /// Sets the event handler.
    pub fn handler<HH>(self, handler: HH) -> Options<'a, HH> {
        let Options { html, width, height, fullscreen, .. } = self;
        Options { html, width, height, fullscreen, handler }
    }
}

impl<'a, H: Handler> Options<'a, H> {
    /// Opens the window.
    pub fn start(self) -> ! {
        Tether::start(self)
    }
}
