#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        extern crate cc;

        fn main() {
            cc::Build::new()
                .cpp(true)
                .file("src/platform/windows/tether.cpp")
                .flag("/ZW")
                .flag("/EHsc")
                .compile("tether.lib");
            println!("cargo:rustc-link-lib=runtimeobject");
        }
    } else {
        fn main() {}
    }
}
