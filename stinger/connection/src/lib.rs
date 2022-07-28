
use std::process;
use std::time::Duration;
use std::thread;

extern crate paho_mqtt as mqtt;

struct Subscription {
    topic: String
}

pub struct Connection {
    pub cli: mqtt::AsyncClient,
    //msg_cb: dyn FnMut(&mqtt::AsyncClient, Option<mqtt::Message>),
}

impl Connection {

    pub fn new(url: String) -> Connection {
        let cli = mqtt::AsyncClient::new(url).unwrap_or_else(|err| {
            println!("Error creating the client: {:?}", err);
            process::exit(1);
        });

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

        // Connect and wait for it to complete or fail
        if let Err(e) = cli.connect(conn_opts).wait() {
            println!("Unable to connect:\n\t{:?}", e);
            process::exit(1);
        }

        // Set a closure to be called whenever the client loses the connection.
        // It will attempt to reconnect, and set up function callbacks to keep
        // retrying until the connection is re-established.
        cli.set_connection_lost_callback(|cli: &mqtt::AsyncClient| {
            println!("Connection lost. Attempting reconnect.");
            thread::sleep(Duration::from_millis(2500));
            cli.reconnect();
        });

        let cb = |_cli, msg: Option<mqtt::Message>| {
            if let Some(msg) = msg {
                let topic = msg.topic();
                let payload_str = msg.payload_str();
                println!("{} - {}", topic, payload_str);
            }
        };

        cli.set_message_callback(cb);

        Connection { 
            cli: cli, 
        //    msg_cb: cb 
        }
    }

    pub fn publish(&mut self, topic: String, message: String) {
        // Create a message and publish it
        let msg = mqtt::Message::new(topic, message, 1);
        let tok = self.cli.publish(msg);
    
        if let Err(e) = tok.wait() {
            println!("Error sending message: {:?}", e);
        }
    }

    pub fn subscribe(&mut self, topic: String, qos: i32) {
        self.cli.subscribe(topic, qos);
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // Disconnect from the broker
        let tok = self.cli.disconnect(None);
        tok.wait().unwrap();
    }
}