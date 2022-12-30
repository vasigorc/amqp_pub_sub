
use amiquip::{Connection, Exchange, Publish};
use amiquip::Result as RabbitResult;

// an abstraction layer to be able to mock AMQP
// components in unit tests
pub trait AMQPPublisher {
    fn publish_message(&self, message: &String) -> Result<String, String>;
}
