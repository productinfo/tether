#![windows_subsystem = "windows"]

//! A pretty silly task list example.
//!
//! Note that the name isn't actually used, because the app doesn't do anything
//! interesting with the data. An interesting extension would be to save the
//! list to a file on `suspend`, and I'm sure that you have ideas that are even
//! more interesting!

extern crate tether;

use std::fmt::Write;
use tether::{Handler, Window};

fn main() {
    tether::builder()
        .html(include_str!("todo.html"))
        .handler(App::new())
        .start();
}

struct App {
    tasks: Vec<Task>,
}

impl App {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }
}

impl Handler for App {
    fn message(&mut self, win: Window, msg: &str) {
        // Commands
        // --------
        // add:{task}
        // toggle-complete:{index}
        // remove:{index}
        // remove-completed:

        let colon = msg.find(':').unwrap();

        match &msg[..colon] {
            "add" => {
                let name = &msg[colon + 1..];
                self.tasks.push(Task { name: name.into(), done: false });
                win.eval(&format!("recv.add({})", tether::escape(name)));
            },
            "toggle-complete" => {
                let i: usize = msg[colon + 1..].parse().unwrap();
                let task = &mut self.tasks[i];
                if task.done {
                    task.done = false;
                    win.eval(&format!("recv.uncomplete({})", i));
                } else {
                    task.done = true;
                    win.eval(&format!("recv.complete({})", i));
                }
            },
            "remove" => {
                let i: usize = msg[colon + 1..].parse().unwrap();
                self.tasks.remove(i);
                win.eval(&format!("recv.remove({})", i));
            },
            "remove-completed" => {
                let mut cmd = "[".to_owned();

                for i in (0..self.tasks.len()).rev() {
                    if self.tasks[i].done {
                        self.tasks.remove(i);
                        write!(cmd, "{},", i).unwrap();
                    }
                }

                cmd.push_str("].forEach(i => { recv.remove(i) })");
                win.eval(&cmd);
            },
            _ => panic!("Unknown command.")
        }
    }
}

#[allow(dead_code)]
struct Task {
    name: String,
    done: bool,
}
