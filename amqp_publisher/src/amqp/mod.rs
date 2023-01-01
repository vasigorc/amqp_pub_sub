use amiquip::Error as RabbitError;
use amiquip::Result as RabbitResult;
use amiquip::{Connection, Exchange, Publish};
#[cfg(test)]
use mockall::automock;

// another abstraction level that we need to decouple
// AMQPMessage from AMQP implementation
pub enum IExchange<'b> {
    Actual(&'b Exchange<'b>),
    // mostly useful for unit tests
    Fake,
}

pub struct AMQPMessage<'a> {
    exchange: &'a IExchange<'a>,
    message: String,
    routing_key: String,
}

// an abstraction layer to be able to mock AMQP
// components in unit tests
#[cfg_attr(test, automock)]
pub trait AMQPPublish {
    fn publish_message<'b>(
        &self,
        message: &String,
        routing_key: &String,
        i_exchange: &IExchange<'b>,
    ) -> Result<String, RabbitError>;
}

impl dyn AMQPPublish {
    fn publish_message<'a>(
        &self,
        message: &String,
        routing_key: &String,
        i_exchange: &IExchange<'a>,
    ) -> Result<String, RabbitError> {
        match i_exchange {
            IExchange::Actual(exchange) => {
                match exchange.publish(Publish::new(message.as_bytes(), routing_key)) {
                    RabbitResult::Err(error) => Err(error),
                    _ => Ok(message.to_string()),
                }
            }
            IExchange::Fake => Ok(message.to_string()),
        }
    }
}
