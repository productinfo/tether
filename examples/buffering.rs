extern crate tether;

use tether::Tether;

fn main() {
    Tether::dispatch(|win| {
        win.eval("document.write('Hello, fellow human!');");
    });

    Tether::builder().start();
}
