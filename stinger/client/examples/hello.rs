
use connection::Connection;

fn main() {
    let mut con = Connection {
        j: 1
    };
    con.publish_one(String::from("Hello world"));
}