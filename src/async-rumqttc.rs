use tokio::{task, time};

use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    task::spawn(async move {
        let mut i = 0;
        loop {
            let payload = vec![1; i];
            let topic = format!("hello/{i}/world");
            let qos = QoS::AtMostOnce;

            let result = client.publish(topic, qos, false, payload).await;
            println!("Publish result = {result:?}");

            time::sleep(Duration::from_secs(3)).await;
            i += 1;
        }
    });

    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                println!("Event = {v:?}");
            }
            Err(e) => {
                println!("Error = {e:?}");
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
