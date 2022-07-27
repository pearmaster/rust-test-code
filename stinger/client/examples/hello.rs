
use connection::Connection;

fn main() {
    let mut con = Connection::new(String::from("tcp://localhost:1883"));
    con.publish_one(String::from("test/3"), String::from("Hello world stuff"));
}