#![windows_subsystem = "windows"]

extern crate tether;

use tether::{Handler, Tether};

fn main() {
    Tether::builder()
        .html("
        	<p>Try closing me! On Windows, try minimizing me, too!</p>
        	<input placeholder='A pointless text field!'>
        ")
        .handler(MyHandler)
        .start();
}

struct MyHandler;

impl Handler for MyHandler {
    fn suspend(&mut self, win: Tether) {
        win.eval("document.body.innerHTML += '<p>Now would be a good time to save things.</p>';");
        println!("Now would be a good time to save things.");
    }
}
