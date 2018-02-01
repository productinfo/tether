#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "stub")] {
        fn main() {}
    } else if #[cfg(target_os = "windows")] {
        extern crate cc;

        fn main() {
            cc::Build::new()
                .cpp(true)
                .file("src/platform/windows/tether.cpp")
                .flag("/ZW")
                .flag("/EHsc")
                .compile("tether.lib");
            println!("cargo:rustc-link-lib=dylib=runtimeobject");
        }
    } else if #[cfg(target_os = "macos")] {
        fn main() {
            println!("cargo:rustc-link-lib=framework=WebKit");
        }
    } else {
        fn main() {}
    }
}
