cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        pub use self::windows::*;
    } else {
        mod gtk;
        pub use self::gtk::*;
    }
    
    //TODO: macOS
    //TODO: Android and iOS would be *really* nice.
}
