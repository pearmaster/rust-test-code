use futures::StreamExt;
use connection::Connection;

pub struct StingerClient {
    connection: Connection,
    thing_callback: Box<dyn FnMut()>,
}

impl StingerClient {
    pub fn new() -> StingerClient {
        let mut con = Connection::new(String::from("tcp://localhost:1883"));
        con.subscribe(String::from("hello/world"), 1);
        con.subscribe(String::from("hello/you"), 1);

        StingerClient {
            connection: con,
            thing_callback: Box::new(|| {}),
        }
    }

    pub fn set_thing_callback(&mut self, cb: impl FnMut() + 'static) {
        self.thing_callback = Box::new(cb);
    }

    pub fn emit_foo(&mut self, value: String) {
        self.connection.publish(String::from("hello/foo"), value)
    }

    pub async fn process(&mut self) {
        while let Some(opt_msg) = self.connection.rx.next().await {
            if let Some(msg) = opt_msg {
                if msg.topic() == "hello/world" {
                    (self.thing_callback)();
                }
            }
        }
    }
}

