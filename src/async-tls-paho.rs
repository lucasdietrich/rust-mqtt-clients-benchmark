use log::LevelFilter;
use paho_mqtt as mqtt;
use std::{env, process, thread};
use tokio;

const TRUST_STORE: &str = "certs/ca.crt.pem";
const KEY_STORE: &str = "certs/client.pem";
const USERNAME: &str = "testuser";
const PASSWORD: &str = "testpassword";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    // Default to secure port with TLS
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtts://localhost:8883".to_string());

    // Resolve and validate trust/key store paths
    let mut trust_store = env::current_dir().unwrap();
    trust_store.push(TRUST_STORE);
    let mut key_store = env::current_dir().unwrap();
    key_store.push(KEY_STORE);

    if !trust_store.exists() {
        eprintln!("Missing trust store file: {:?}", trust_store);
        process::exit(1);
    }

    if !key_store.exists() {
        eprintln!("Missing key store file: {:?}", key_store);
        process::exit(1);
    }

    println!("Connecting to the MQTT server at '{}'", &host);

    // Build SSL options
    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(trust_store)
        .unwrap()
        .key_store(key_store)
        .unwrap()
        .finalize();

    // Create connection options with SSL and authentication
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .ssl_options(ssl_opts)
        .user_name(USERNAME)
        .password(PASSWORD)
        .clean_session(true)
        .finalize();

    // Create the client
    let client = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        eprintln!("Error creating the client: {}", err);
        process::exit(1);
    });

    loop {
        // Try to connect with secure options
        let result = client.connect(conn_opts.clone()).await;
        println!("Connecting to the broker: {:?}", result);
        if result.is_err() {
            thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }

        loop {
            // Send a secure message
            let msg = mqtt::Message::new("test", "Hello secure MQTT world!", mqtt::QOS_1);
            let result = client.publish(msg).await;
            println!("Publishing the message: {:?}", result);
            if result.is_err() {
                break;
            }

            thread::sleep(std::time::Duration::from_secs(3));
        }
    }

    // You won't reach this due to infinite loop, but good practice:
    // client.disconnect(None).await.expect("Error disconnecting");
}
