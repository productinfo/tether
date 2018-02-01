#![windows_subsystem = "windows"]

extern crate tether;

use tether::Tether;

fn main() {
    Tether::builder()
        .html(include_str!("todo.html"))
        .handler(|_, msg: &str| {
            println!("Received message: {}", msg);
        })
        .start();
}
