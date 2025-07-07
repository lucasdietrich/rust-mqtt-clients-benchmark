use std::time::Duration;

use rust_mqtt_eval::{paho::Paho, traits::Parameters};
use tokio::io::{AsyncBufRead, AsyncRead};

#[tokio::main]
async fn main() {
    let (mut paho, receiver) = Paho::new(Parameters::default());

    paho.connect().await.unwrap();
    paho.subscribe("test/topic").await.unwrap();

    tokio::spawn(rx(receiver));
    tokio::spawn(tx(paho));

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn rx(rx: paho_mqtt::AsyncReceiver<Option<paho_mqtt::Message>>) {
    while let Ok(msg) = rx.recv().await {
        if let Some(message) = msg {
            println!("Received message: {:?}", message);
        } else {
            println!("No message received");
        }
    }
}

async fn tx(mut paho: Paho) {
    use tokio::io::{self, AsyncBufReadExt};
    let mut i = 0;

    let stdin = io::stdin();
    let mut lines = io::BufReader::new(stdin).lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        let msg = format!("Message {}: {}", i, line);
        let result = paho.publish("test/topic", msg.as_bytes()).await;
        println!(
            "Published message to topic 'test/topic': {:?}",
            result
        );
        i += 1;
    }
}
