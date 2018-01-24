extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

use {Handler, Options, Tether};
use self::gtk::{ContainerExt, WidgetExt, GtkWindowExt, WindowType};
use self::webkit2gtk::{
    UserContentInjectedFrames,
    UserContentManager,
    UserContentManagerExt,
    UserScript,
    UserScriptInjectionTime,
    WebContext,
    WebView,
    WebViewExt,
    WebViewExtManual
};
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static WEBVIEW: RefCell<Option<WebView>> = RefCell::new(None);
}

pub fn start<H: Handler>(opts: Options<H>) {
    let handler = Rc::new(RefCell::new(opts.handler));

    // Initialize GTK.

    gtk::init().unwrap();

    // Create the window.

    let window = gtk::Window::new(WindowType::Toplevel);

    window.set_default_size(opts.width as i32, opts.height as i32);

    // Make the web view.

    let context = WebContext::get_default().unwrap();
    let scripts = UserContentManager::new();
    let webview = WebView::new_with_context_and_user_content_manager(
        &context,
        &scripts,
    );

    // Setup the window title.

    window.set_title("");
    webview.connect_title_changed({
        let window = window.clone();
        let webview = webview.clone();
        move || {
            window.set_title(match webview.get_title() {
                Some(ref title) => &title,
                None => "",
            });
        }
    });

    // Register the message event.

    scripts.connect_script_message_received({
        let handler = handler.clone();
        move |_, result| {
            let ctx = result.get_global_context();
            let val = result.get_value();

            if let (Some(val), Some(ctx)) = (val, ctx) {
                if let Some(msg) = val.to_string(&ctx) {
                    handler.borrow_mut().message(
                        unsafe { Tether::new() },
                        &msg,
                    )
                }
            }
        }
    });

    scripts.register_script_message_handler("x");

    scripts.add_script(&UserScript::new(
        "window.tether = function (s) { window.webkit.messageHandlers.x.postMessage(s); }",
        UserContentInjectedFrames::TopFrame,
        UserScriptInjectionTime::Start,
        &[], // Whitelisted URIs.
        &[], // Blacklisted URIs.
    ));

    // Register the close event.

    window.connect_delete_event({
        let handler = handler.clone();
        move |_, _| {
            handler.borrow_mut().close();
            gtk::main_quit();
            gtk::Inhibit(false)
        }
    });

    // Open the window.

    webview.load_html(opts.html, None);
    window.add(&webview);
    window.show_all();

    if opts.fullscreen {
        window.fullscreen();
    }

    // Globally store the webview reference.

    WEBVIEW.with(move |global| {
        *global.borrow_mut() = Some(webview);
    });

    // Run the main loop.

    gtk::main();
}

pub fn load(html: &str) {
    WEBVIEW.with(|webview| {
        webview
            .borrow()
            .as_ref()
            .unwrap()
            .load_html(html, None);
    });
}

pub fn eval(js: &str) {
    WEBVIEW.with(|webview| {
        webview
            .borrow()
            .as_ref()
            .unwrap()
            .run_javascript(js);
    });
}

pub fn dispatch<F: FnOnce(Tether) + Send + 'static>(f: F) {
    let mut f = Some(f);
    glib::idle_add(move || {
        f.take().unwrap()(unsafe { Tether::new() });
        glib::Continue(false)
    });
}
