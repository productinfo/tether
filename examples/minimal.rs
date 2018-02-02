#![windows_subsystem = "windows"]

extern crate tether;

fn main() {
    tether::builder()
        .html("
            <title>This is a pretty cool window!</title>
            <button onclick=\"tether('Hello, world!')\">Click me!</button>
        ")
        .handler(|_, msg: &str| {
            println!("Received message: {}", msg);
        })
        .start();
}
