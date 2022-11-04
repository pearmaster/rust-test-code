extern crate paho_mqtt as mqtt;
use std::process;
use futures::{executor::block_on, StreamExt};

fn main() {
    let mut cli = mqtt::AsyncClient::new("tcp://localhost:1883").unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let mut strm = cli.get_stream(25);

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .finalize();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts).wait() {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    cli.subscribe("echo/to", 2);

    block_on(
        async {
            while let Some(msg_opt) = strm.next().await {
                if let Some(msg) = msg_opt {
                    let topic: String = String::from("echo/from");
                    let payload: String = msg.payload_str().to_string();
                    let qos: i32 = 2;
                    let pub_msg = mqtt::Message::new(topic, payload, qos);
                    cli.publish(pub_msg);
                }
                else {
                    // A "None" means we were disconnected. Try to reconnect...
                    println!("Lost connection.");
                }
            }
        }
    )
}
