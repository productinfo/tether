extern crate tether;

use tether::Tether;

fn main() {
    Tether::builder()
        .html("<button onclick=\"tether('Hello, world!')\">Click me!</button>")
        .handler(|_, msg: &str| {
            println!("Received message: {}", msg);
        })
        .start();
}
