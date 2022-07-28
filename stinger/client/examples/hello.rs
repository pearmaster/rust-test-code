
use std::{thread, time::Duration};

use connection::Connection;

fn main() {
    let mut con = Connection::new(String::from("tcp://localhost:1883"));
    con.publish(String::from("test/5"), String::from("Hello world stuff"));

    con.subscribe(String::from("hello/world"), 1);

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
    // Ctrl-C to stop
}