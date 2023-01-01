
use amiquip::{Connection, Exchange, Publish};
use amiquip::Result as RabbitResult;
#[cfg(test)]
use mockall::automock;

// an abstraction layer to be able to mock AMQP
// components in unit tests
#[cfg_attr(test, automock)]
pub trait AMQPPublish {
    fn publish_message(&self, message: &String) -> Result<String, String>;
}
