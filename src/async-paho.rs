use log::LevelFilter;
use paho_mqtt as mqtt;
use std::{env, process, thread};

/////////////////////////////////////////////////////////////////////////////

#[tokio::main(flavor = "current_thread")]
async fn main() {
    simple_logger::SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    // Command-line option(s)
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    println!("Connecting to the MQTT server at '{}'", host);

    // Create the client
    let client = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        println!("Error creating the client: {}", err);
        process::exit(1);
    });

    loop {
        // Connect with default options and wait for it to complete or fail
        // The default is an MQTT v3.x connection.
        let result = client.connect(None).await;
        println!("Connecting to the broker: {:?}", result);
        if result.is_err() {
            thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }

        loop {
            // Create a message and publish it
            let msg = mqtt::Message::new("test", "Hello Rust MQTT world!", mqtt::QOS_0);
            let result = client.publish(msg).await;
            println!("Publishing the message: {:?}", result);
            if result.is_err() {
                break;
            }

            thread::sleep(std::time::Duration::from_secs(3));
        }
    }

    // Disconnect from the broker
    // println!("Disconnecting");
    // client.disconnect(None).await.expect("Error disconnecting");
}
