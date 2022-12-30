# Basic AMQP Pub-Sub example
This project features a basic pub-sub implementation from the [RabbitMQ in Depth](https://www.amazon.ca/RabbitMQ-Depth-Gavin-M-Roy/dp/1617291005).

It is based on the following components:

1. [RabbitMQ docker client](https://hub.docker.com/_/rabbitmq)
2. A publisher written in [Rust](https://www.rust-lang.org/)
3. A consumer written in [Scala 3](https://www.scala-lang.org/api/3.2.1/) and using [ZIO](https://zio.dev/)
4. [Docker compose](https://docs.docker.com/compose/gettingstarted/) as a glue for a running system