
use futures::{executor::block_on};

use connection::Connection;

fn main() {
    let mut con = Connection::new(String::from("tcp://localhost:1883"));
    
    con.publish(String::from("test/5"), String::from("Hello world stuff"));

    con.subscribe(String::from("hello/world"), 1);

    block_on(async {
        while let Some(opt_msg) = con.rx.next().await {
            if let Some(msg) = opt_msg {
                println!("{}", msg);
            }
        }
    });
    // Ctrl-C to stop
}