use async_trait::async_trait;
use paho_mqtt as paho;

use crate::traits::{MqttDriver, MqttError, Parameters};

const USERNAME: &str = "testuser";
const PASSWORD: &str = "testpassword";

enum State {
    Uninitialized,
    Initialized(paho::AsyncClient),
}

pub struct Paho {
    first_connect: bool,
    paho: paho::AsyncClient,
    opts: paho::ConnectOptions,
}

impl Paho {
    pub fn new(params: Parameters) -> (Paho, paho_mqtt::AsyncReceiver<Option<paho_mqtt::Message>>) {
        // Create connection options with SSL and authentication
        let conn_opts = paho::ConnectOptionsBuilder::new()
            .user_name(USERNAME)
            .password(PASSWORD)
            .clean_session(true)
            .finalize();

        let host = "mqtts://localhost:8883";

        // Create the client
        let mut client = paho::AsyncClient::new(params.host).unwrap();

        let receiver = client.get_stream(100);

        let paho = Paho {
            first_connect: true,
            paho: client,
            opts: conn_opts,
        };

        (paho, receiver)
    }

    async fn try_connect(&mut self) -> Result<(), MqttError> {
        match self.paho.is_connected() {
            true => Ok(()),
            false => {
                if self.first_connect {
                    self.paho.connect(self.opts.clone()).await.map_err(|e| {
                        MqttError::ConnectionFailed(format!(
                            "Failed to connect to MQTT broker: {}",
                            e
                        ))
                    })?;
                    self.first_connect = false;
                    Ok(())
                } else {
                    Err(MqttError::ConnectionFailed(
                        "Failed to connect to MQTT broker".to_string(),
                    ))
                }
            }
        }
    }

    pub async fn connect(&mut self) -> Result<(), MqttError> {
        self.paho.connect(self.opts.clone()).await.map_err(|e| {
            MqttError::ConnectionFailed(format!(
                "Failed to connect to MQTT broker: {}",
                e
            ))
        })?;
        Ok(())
    }

    pub async fn subscribe(&mut self, topic: &str) -> Result<(), MqttError> {
        self.try_connect().await?;
        self.paho.subscribe(topic, paho::QOS_0).await.map_err(|e| {
            MqttError::SubscriptionFailed(format!(
                "Failed to subscribe to topic {}: {}",
                topic, e
            ))
        })?;
        Ok(())
    }

    pub async fn publish(&mut self, topic: &str, payload: &[u8]) -> Result<(), MqttError> {
        println!("is connected: {}", self.paho.is_connected());
        self.try_connect().await?;
        let msg = paho::Message::new(topic, payload, paho::QOS_0);
        self.paho.publish(msg).await.map_err(|e| {
            MqttError::PublishFailed(format!(
                "Failed to publish message to topic {}: {}",
                topic, e
            ))
        })?;
        Ok(())
    }
}

// #[async_trait]
// impl MqttDriver for Paho {
//     type Message = Vec<u8>;

//     async fn connect(&mut self, options: Parameters) -> Result<(), MqttError> {
//         todo!()
//     }

//     async fn subscribe(&mut self, topic: &str) -> Result<(), MqttError>{
//         todo!()
//     }

//     async fn publish(&mut self) -> Result<(), MqttError>{
//         todo!()
//     }

//     async fn receive(&mut self) -> Result<Self::Message, MqttError>{

//     }
// }
