#![windows_subsystem = "windows"]

//! An example that uses a server to load a WebAssembly file.
//!
//! The details are in the comments! Note that the general idea of using a web
//! server to serve embedded files can also be used for things like images,
//! video and audio, rather than just WebAssembly. I guess you could also use
//! WebSockets to do even more convoluted stuff, too.

extern crate tiny_http;
extern crate tether;

use std::thread;
use tether::{Handler, Window};
use tiny_http::{Header, Method, Response, Server, StatusCode};

fn main() {
    // Using your favorite web framework, open a server on 127.0.0.1:0.
    // Because the port number is zero, the OS gets to choose the port
    let server = Server::http("127.0.0.1:0").unwrap();

    // Then, find out what port the OS chose, to pass it to the JavaScript.
    let addr = server.server_addr();

    // The window takes up the main thread, so run the server on a new thread.
    thread::spawn(move || {
        for req in server.incoming_requests() {
            let res = match (req.method(), req.url()) {
                (&Method::Get, "/life.wasm") => {
                    let life = include_bytes!("life.wasm");

                    Response::new(
                        StatusCode(200),
                        vec![
                            // Make sure the MIME type is correct.
                            header("Content-Type", "application/wasm"),
                            // All endpoints must allow for cross-origin requests.
                            header("Access-Control-Allow-Origin", "*"),
                        ],
                        life as &[u8],
                        None,
                        None,
                    )
                },
                _ => {
                    Response::new(
                        StatusCode(404),
                        vec![],
                        &[] as &[u8],
                        None,
                        None,
                    )
                }
            };

            req.respond(res).unwrap();
        }
    });

    tether::builder()
        .html(include_str!("binary.html"))
        .handler(App { port: addr.port() })
        .start();
}

struct App {
    port: u16,
}

impl Handler for App {
    fn message(&mut self, win: Window, msg: &str) {
        match msg {
            "load" => {
                // The JavaScript side is ready, so we give it the port.
                win.eval(&format!("main({})", self.port));
            },
            _ => panic!("Unknown message."),
        }
    }
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).unwrap()
}
