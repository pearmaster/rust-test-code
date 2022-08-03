
use futures::{executor::block_on};

use client::StingerClient;

fn print_msg(i: u32) {
    println!("Thing {}", i);
}

fn main() {
    let mut client = StingerClient::new();
    client.set_thing_callback(print_msg);
    block_on(async {
        client.process().await;
    });
    // Ctrl-C to stop
}