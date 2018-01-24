cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use self::linux::*;
    } else {
        //TODO: Windows
        //TODO: macOS
        //TODO: Android
        //TODO: iOS
    }
}
