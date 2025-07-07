use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MqttError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Subscription failed: {0}")]
    SubscriptionFailed(String),

    #[error("Publish failed: {0}")]
    PublishFailed(String),

    #[error("Receive failed: {0}")]
    ReceiveFailed(String),
}

pub struct Parameters {
    // host, port
    pub host: String,
    // send timeout
    // qos
    // keep alive
    // SSL
    // retries ?
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            host: "mqtt://localhost:1883".to_string(),
        }
    }
}

pub trait MessageTrait {
    fn get_payload(&self) -> &[u8];
    fn get_len(&self) -> usize;
}

#[async_trait]
pub trait MqttDriver {
    type Message: MessageTrait;

    async fn connect(&mut self, params: Parameters) -> Result<(), MqttError>;

    async fn subscribe(&mut self, topic: &str) -> Result<(), MqttError>;

    async fn publish(&mut self) -> Result<(), MqttError>;

    async fn receive(&mut self) -> Result<Self::Message, MqttError>;
}

// #[async_trait]
// trait MqttTrait {
//     type Driver: MqttDriver;

//     async fn connect(&mut self, options: Parameters) -> Result<(), MqttError>;

//     async fn subscribe(&mut self, topic: &str) -> Result<(), MqttError>;

//     async fn publish(&mut self) -> Result<(), MqttError>;

//     async fn receive(&mut self) -> Result<<Self::Driver as MqttDriver>::Message, MqttError>;
// }
