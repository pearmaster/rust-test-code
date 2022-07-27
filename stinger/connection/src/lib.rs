
use std::process;
use std::time::Duration;

extern crate paho_mqtt as mqtt;

pub struct Connection {
    pub cli: mqtt::AsyncClient
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

        Connection { cli: cli }
    }

    pub fn publish_one(&mut self, topic: String, message: String) {
        

    
        // Create a message and publish it
        let msg = mqtt::Message::new(topic, message, 1);
        let tok = self.cli.publish(msg);
    
        if let Err(e) = tok.wait() {
            println!("Error sending message: {:?}", e);
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // Disconnect from the broker
        let tok = self.cli.disconnect(None);
        tok.wait().unwrap();
    }
}