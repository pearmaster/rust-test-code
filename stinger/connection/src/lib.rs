
use std::process;
use std::time::Duration;

extern crate paho_mqtt as mqtt;

pub struct Connection {
    pub j: u32
}

impl Connection {
    pub fn publish_one(&mut self, message: String) {
            // Create a client & define connect options
            let cli = mqtt::AsyncClient::new("tcp://localhost:1883").unwrap_or_else(|err| {
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
        
            // Create a message and publish it
            let msg = mqtt::Message::new("test", message, 1);
            let tok = cli.publish(msg);
        
            if let Err(e) = tok.wait() {
                println!("Error sending message: {:?}", e);
            }
        
            // Disconnect from the broker
            let tok = cli.disconnect(None);
            tok.wait().unwrap();
    }
}