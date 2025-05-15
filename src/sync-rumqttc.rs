use rumqttc::{Client, LastWill, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    let will = LastWill::new("hello/world", "good bye", QoS::AtMostOnce, false);
    mqttoptions
        .set_keep_alive(Duration::from_secs(5))
        .set_last_will(will);

    let (client, mut connection) = Client::new(mqttoptions, 10);
    thread::spawn(move || {
        let mut i = 0;
        loop {
            let payload = vec![1; i];
            let topic = format!("hello/{i}/world");
            let qos = QoS::AtMostOnce;

            let result = client.publish(topic, qos, true, payload);
            println!("Publish result = {result:?}");

            thread::sleep(Duration::from_secs(3));
            i += 1;
        }
    });

    for (i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(notif) => {
                println!("{i}. Notification = {notif:?}");
            }
            Err(error) => {
                println!("{i}. Notification = {error:?}");
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    println!("Done with the stream!!");
}
