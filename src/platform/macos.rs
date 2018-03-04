#![allow(non_snake_case, non_upper_case_globals)]

use {Handler, Options, Window};
use cocoa::appkit::{
    NSApplicationActivationPolicyRegular,
    NSApplicationTerminateReply,
    NSBackingStoreBuffered,
    NSWindowStyleMask,
};
use cocoa::base::{BOOL, YES, NO, id, nil};
use cocoa::foundation::{
    NSInteger,
    NSRect,
    NSPoint,
    NSSize,
    NSUInteger,
};
use dispatch::Queue;
use objc::{Encode, Encoding};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use std::{process, ptr};
use std::borrow::Cow;
use std::cell::Cell;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

thread_local! {
    static WEBVIEW: Cell<*const Object> = Cell::new(ptr::null());
}

pub fn start<H: Handler>(opts: Options<H>) -> ! {
    unsafe {
        let NSApplication = Class::get("NSApplication").unwrap();
        let NSAutoreleasePool = Class::get("NSAutoreleasePool").unwrap();
        let NSObject = Class::get("NSObject").unwrap();
        let NSWindow = Class::get("NSWindow").unwrap();
        let RKAppDelegate = register_delegate(NSObject);

        // Create things.

        let pool: id = msg_send![NSAutoreleasePool, alloc];
        let app: id = msg_send![NSApplication, sharedApplication];
        let delegate: id = msg_send![RKAppDelegate, alloc];

        let pool: id = msg_send![pool, init];
        let delegate: id = msg_send![delegate, init];

        // Add the main menu.

        add_menus(app);

        // Pass in the options.

        let html = NSString::new(opts.html);

        let handler = Speedwagon {
            data: Box::into_raw(Box::new(opts.handler)) as *mut c_void,
            message: { extern "C" fn message<H: Handler>(h: *mut c_void, s: &str) {
                unsafe {
                    let h = h as *mut H;
                    (*h).message(Window::new(), s)
                }
            }; message::<H> },
            suspend: { extern "C" fn suspend<H: Handler>(h: *mut c_void) {
                unsafe {
                    let h = h as *mut H;
                    (*h).suspend(Window::new())
                }
            }; suspend::<H> },
            drop: { extern "C" fn drop<H: Handler>(h: *mut c_void) {
                unsafe {
                    let h = h as *mut H;
                    Box::from_raw(h);
                }
            }; drop::<H> },
        };

        let frame = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(opts.width as f64, opts.height as f64));

        msg_send![html.raw, retain];

        ptr::write((*delegate).get_mut_ivar("fullscreen"), opts.fullscreen);
        ptr::write((*delegate).get_mut_ivar("frame"), frame);
        ptr::write((*delegate).get_mut_ivar("html"), html.raw);
        ptr::write((*delegate).get_mut_ivar("handler"), handler);

        // Configure the app.

        msg_send![NSWindow, setAllowsAutomaticWindowTabbing: NO];
        msg_send![app, setDelegate:delegate];
        msg_send![app, setActivationPolicy:NSApplicationActivationPolicyRegular];

        // Run the app.

        msg_send![app, run];

        // Release things.

        WEBVIEW.with(|cell| {
            let old = cell.replace(ptr::null());
            if !old.is_null() {
                msg_send![old, release];
            }
        });

        msg_send![delegate, release];
        msg_send![pool, release];
    }

    process::exit(0);
}

pub fn load(html: &str) {
    WEBVIEW.with(|cell| {
        let webview = cell.get();
        if !webview.is_null() {
            unsafe {
                msg_send![webview,
                    loadHTMLString: NSString::new(html)
                    baseURL: nil];
            }
        }
    })
}

pub fn eval(js: &str) {
    WEBVIEW.with(|cell| {
        let webview = cell.get();
        if !webview.is_null() {
            unsafe {
                msg_send![webview,
                    evaluateJavaScript:NSString::new(js)
                    completionHandler:nil];
            }
        }
    })
}

pub fn dispatch<F: FnOnce(Window) + Send + 'static>(f: F) {
    Queue::main().async(move || {
        f(unsafe { Window::new() });
    });
}

unsafe fn register_delegate(NSObject: &Class) -> &'static Class {
    let mut decl = ClassDecl::new("RKAppDelegate", NSObject).unwrap();

    decl.add_method(sel!(applicationDidFinishLaunching:), application_did_finish_launching as extern "C" fn(&mut Object, Sel, *mut Object));
    decl.add_method(sel!(applicationShouldTerminateAfterLastWindowClosed:), should_app_kys as extern "C" fn(&mut Object, Sel, *mut Object) -> BOOL);
    decl.add_method(sel!(applicationShouldTerminate:), app_will_kys as extern "C" fn(&mut Object, Sel, *mut Object) -> c_int);
    decl.add_method(sel!(dealloc), dealloc_delegate as extern "C" fn(&mut Object, Sel));
    decl.add_method(sel!(userContentController:didReceiveScriptMessage:), receive_message as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object));
    decl.add_method(sel!(observeValueForKeyPath:ofObject:change:context:), handle_change as extern "C" fn(&mut Object, Sel, *mut Object, *mut Object, *mut Object, *mut c_void));

    decl.add_ivar::<bool>("fullscreen");
    decl.add_ivar::<NSRect>("frame");
    decl.add_ivar::<id>("html");
    decl.add_ivar::<Speedwagon>("handler");

    decl.register()
}

extern "C" fn application_did_finish_launching(this: &mut Object, _: Sel, _: *mut Object) {
    unsafe {
        let NSWindow = Class::get("NSWindow").unwrap();
        let WKWebView = Class::get("WKWebView").unwrap();
        let WKUserScript = Class::get("WKUserScript").unwrap();
        let WKWebViewConfiguration = Class::get("WKWebViewConfiguration").unwrap();

        let str_x = NSString::new("x");
        let str_script = NSString::new("window.tether = function (s) { window.webkit.messageHandlers.x.postMessage(s); };");

        // Create things.

        let window: id = msg_send![NSWindow, alloc];
        let webview: id = msg_send![WKWebView, alloc];
        let config: id = msg_send![WKWebViewConfiguration, new];
        let script: id = msg_send![WKUserScript, alloc];

        // Register the message event stuff.

        let cc: id = msg_send![config, userContentController];

        let script: id = msg_send![script,
            initWithSource:str_script
            injectionTime:0 as NSInteger
            forMainFrameOnly:YES];

        msg_send![cc, addScriptMessageHandler:&*this name:str_x];
        msg_send![cc, addUserScript:script];

        // Make the web view.

        let webview: id = msg_send![webview,
            initWithFrame:NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(0.0, 0.0))
            configuration:config];

        msg_send![webview,
            addObserver:&*this
            forKeyPath:NSString::new("title")
            options:0 as c_int
            context:ptr::null_mut::<c_void>()];

        msg_send![webview,
            loadHTMLString:*this.get_ivar::<id>("html")
            baseURL:nil];

        WEBVIEW.with(|cell| {
            let old = cell.replace(webview);
            if !old.is_null() {
                msg_send![old, release];
            }
        });

        // Make the window.

        let mut window_style = NSWindowStyleMask::NSTitledWindowMask
            | NSWindowStyleMask::NSClosableWindowMask
            | NSWindowStyleMask::NSMiniaturizableWindowMask
            | NSWindowStyleMask::NSResizableWindowMask;

        if *this.get_ivar::<bool>("fullscreen") {
            window_style |= NSWindowStyleMask::NSFullScreenWindowMask;
        }

        let frame: NSRect = *this.get_ivar::<NSRect>("frame");

        let window: id = msg_send![window,
            initWithContentRect:frame
            styleMask:window_style
            backing:NSBackingStoreBuffered
            defer:NO];

        msg_send![window, setContentMinSize:frame.size];
        msg_send![window, setContentView:webview];
        msg_send![window, center];
        msg_send![window, makeKeyAndOrderFront:nil];

        // Free all the things.

        msg_send![config, release];
        msg_send![script, release];
    }
}

extern "C" fn app_will_kys(this: &mut Object, _: Sel, _: *mut Object) -> c_int {
    unsafe {
        let handler = this.get_mut_ivar::<Speedwagon>("handler");
        ((*handler).suspend)((*handler).data);
    }

    NSApplicationTerminateReply::NSTerminateNow as c_int
}

extern "C" fn should_app_kys(_: &mut Object, _: Sel, _: *mut Object) -> BOOL {
    YES
}

extern "C" fn receive_message(this: &mut Object, _: Sel, _: *mut Object, msg: *mut Object) {
    unsafe {
        let NSString = Class::get("NSString").unwrap();
        let handler = this.get_mut_ivar::<Speedwagon>("handler");
        let body: id = msg_send![msg, body];

        if YES == msg_send![body, isKindOfClass:NSString] {
            let s = NSString::from_raw(body);
            ((*handler).message)((*handler).data, &s.as_str());
        }
    }
}

extern "C" fn dealloc_delegate(this: &mut Object, _: Sel) {
    //TODO: The deallocator never seems to be called. ;_;
    unsafe {
        let NSObject = Class::get("NSObject").unwrap();
        msg_send![*this.get_mut_ivar::<id>("html"), release];
        ptr::drop_in_place(this.get_mut_ivar::<Speedwagon>("handler"));
        msg_send![super(this, NSObject), release];
    }
}

extern "C" fn handle_change(_: &mut Object, _: Sel, _: *mut Object, obj: *mut Object, _: *mut Object, _: *mut c_void) {
    unsafe {
        let title: id = msg_send![obj, title];
        let window: id = msg_send![obj, window];
        msg_send![window, setTitle:title];
    }
}

struct NSString { raw: id }

impl NSString {
    fn new(s: &str) -> Self {
        let NSString = Class::get("NSString").unwrap();

        let raw = unsafe {
            let raw: id = msg_send![NSString, alloc];
            let raw: id = msg_send![raw,
                initWithBytes:s.as_ptr()
                length:s.len()
                encoding:4 as NSUInteger];
            raw
        };

        Self { raw }
    }

    unsafe fn concat(&self, other: id) -> Self {
        let raw: id = msg_send![self.raw, stringByAppendingString:other];
        Self { raw }
    }

    unsafe fn from_raw(raw: id) -> Self {
        msg_send![raw, retain];
        Self { raw }
    }

    fn as_str(&self) -> Cow<str> {
        unsafe {
            let ptr: *const c_char = msg_send![self.raw, UTF8String];
            CStr::from_ptr(ptr).to_string_lossy()
        }
    }
}

impl Clone for NSString {
    fn clone(&self) -> Self {
        unsafe {
            msg_send![self.raw, retain];
        }

        Self { raw: self.raw }
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe {
            msg_send![self.raw, release];
        }
    }
}

#[repr(C)]
struct Speedwagon {
    data: *mut c_void,
    message: extern "C" fn(*mut c_void, &str),
    suspend: extern "C" fn(*mut c_void),
    drop: extern "C" fn(*mut c_void),
}

impl Drop for Speedwagon {
    fn drop(&mut self) {
        (self.drop)(self.data);
    }
}

unsafe impl Encode for Speedwagon {
    fn encode() -> Encoding { unsafe { Encoding::from_str("{spw=^v^?^?^?}") } }
}

unsafe fn add_menus(app: id) {
    //TODO: Memory management might not have been done correctly here.

    // {APP NAME}
    //     Hide {APP NAME}   Cmd + H
    //     Hide Others       Cmd + Opt + H
    //     Show All
    //     --------
    //     Quit {APP NAME}
    // Edit
    //     Undo   Cmd + Z
    //     Redo   Cmd + Shift + Z
    //     --------
    //     Cut          Cmd + X
    //     Copy         Cmd + C
    //     Paste        Cmd + V
    //     Delete
    //     Select All   Cmd + A
    // Window
    //     Close                      Cmd + W
    //     Minimize                   Cmd + M
    //     (Enter|Exit) Full Screen   Cmd + Ctrl + F

    // MAIN MENU

    let NSMenu = Class::get("NSMenu").unwrap();
    let NSMenuItem = Class::get("NSMenuItem").unwrap();
    let NSProcessInfo = Class::get("NSProcessInfo").unwrap();

    let process: id = msg_send![NSProcessInfo, processInfo];
    let appname: id = msg_send![process, processName];

    let str_edit = NSString::new("Edit");
    let str_window = NSString::new("Window");
    let str_empty = NSString::new("");
    let str_mainmenu = NSString::new("MainMenu");
    let str_hide_others = NSString::new("Hide Others");
    let str_close = NSString::new("Close");
    let str_show_all = NSString::new("Show All");
    let str_minimize = NSString::new("Minimize");
    let str_toggle_fullscreen = NSString::new("Toggle Full Screen");
    let str_select_all = NSString::new("Select All");
    let str_cut = NSString::new("Cut");
    let str_copy = NSString::new("Copy");
    let str_paste = NSString::new("Paste");
    let str_delete = NSString::new("Delete");
    let str_undo = NSString::new("Undo");
    let str_redo = NSString::new("Redo");
    let str_hide_appname = NSString::new("Hide ").concat(appname);
    let str_quit_appname = NSString::new("Quit ").concat(appname);
    let str_a = NSString::new("a");
    let str_c = NSString::new("c");
    let str_f = NSString::new("f");
    let str_h = NSString::new("h");
    let str_m = NSString::new("m");
    let str_q = NSString::new("q");
    let str_v = NSString::new("v");
    let str_w = NSString::new("w");
    let str_x = NSString::new("x");
    let str_z = NSString::new("z");

    let menu: id = msg_send![NSMenu, alloc];
    let menu: id = msg_send![menu, initWithTitle:str_mainmenu.raw];

    let item_app: id = msg_send![menu, addItemWithTitle:appname action:nil keyEquivalent:str_empty.raw];
    let item_edit: id = msg_send![menu, addItemWithTitle:str_edit.raw action:nil keyEquivalent:str_empty.raw];
    let item_window: id = msg_send![menu, addItemWithTitle:str_window.raw action:nil keyEquivalent:str_empty.raw];

    // APP MENU

    let submenu: id = msg_send![NSMenu, alloc];
    let submenu: id = msg_send![submenu, initWithTitle:appname];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_hide_appname.raw
        action:sel!(hide:)
        keyEquivalent:str_h.raw];

    let item: id = msg_send![submenu,
        addItemWithTitle:str_hide_others.raw
        action:sel!(hideOtherApplications:)
        keyEquivalent:str_h.raw];
    msg_send![item, setKeyEquivalentModifierMask:(3 as c_int) << 19];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_show_all.raw
        action:sel!(unhideAllApplications:)
        keyEquivalent:str_empty.raw];

    let sep: id = msg_send![NSMenuItem, separatorItem];
    msg_send![submenu, addItem:sep];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_quit_appname.raw
        action:sel!(terminate:)
        keyEquivalent:str_q.raw];

    msg_send![menu, setSubmenu:submenu forItem:item_app];
    msg_send![submenu, release];

    // EDIT MENU

    let submenu: id = msg_send![NSMenu, alloc];
    let submenu: id = msg_send![submenu, initWithTitle:str_edit.raw];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_undo.raw
        action:sel!(undo:)
        keyEquivalent:str_z.raw];

    let item: id = msg_send![submenu,
        addItemWithTitle:str_redo.raw
        action:sel!(redo:)
        keyEquivalent:str_z.raw];
    msg_send![item, setKeyEquivalentModifierMask:(9 as c_int) << 17];

    let sep: id = msg_send![NSMenuItem, separatorItem];
    msg_send![submenu, addItem:sep];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_cut.raw
        action:sel!(cut:)
        keyEquivalent:str_x.raw];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_copy.raw
        action:sel!(copy:)
        keyEquivalent:str_c.raw];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_paste.raw
        action:sel!(paste:)
        keyEquivalent:str_v.raw];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_delete.raw
        action:sel!(delete:)
        keyEquivalent:str_empty.raw];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_select_all.raw
        action:sel!(selectAll:)
        keyEquivalent:str_a.raw];

    msg_send![menu, setSubmenu:submenu forItem:item_edit];
    msg_send![submenu, release];

    // WINDOW MENU

    let submenu: id = msg_send![NSMenu, alloc];
    let submenu: id = msg_send![submenu, initWithTitle:str_window.raw];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_close.raw
        action:sel!(close)
        keyEquivalent:str_w.raw];

    let _: id = msg_send![submenu,
        addItemWithTitle:str_minimize.raw
        action:sel!(miniaturize:)
        keyEquivalent:str_m.raw];

    let item: id = msg_send![submenu,
        addItemWithTitle:str_toggle_fullscreen.raw
        action:sel!(toggleFullScreen:)
        keyEquivalent:str_f.raw];
    msg_send![item, setKeyEquivalentModifierMask:(5 as c_int) << 18];

    msg_send![menu, setSubmenu:submenu forItem:item_window];
    msg_send![app, setWindowsMenu:submenu];
    msg_send![submenu, release];

    // END

    msg_send![app, setMainMenu:menu];
    msg_send![menu, release];
}
