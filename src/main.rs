use lapin::{
    options::*,
    publisher_confirm::Confirmation,
    types::FieldTable,
    BasicProperties,
    Connection,
    ConnectionProperties,
};
use tokio_amqp::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default().with_tokio()
    ).await?; //

    let channel = conn.create_channel().await?;

    let queue = "dev-queue";
    channel.queue_declare(
        queue,
        QueueDeclareOptions { durable: true, ..Default::default() },
        FieldTable::default()
    ).await?;
    let payload = b"Hello, RabbitMQ!1111";

    let confirm = channel.basic_publish(
        "",
        queue,
        BasicPublishOptions::default(),
        payload,
        BasicProperties::default()
    ).await?.await?;

    match confirm {
        Confirmation::NotRequested => println!("Message published without confirmation"),
        Confirmation::Nack(_) => println!("Message was not acknowledged"),
        Confirmation::Ack(_) => println!("Message acknowledged"),
    }
    if let Err(err) = main() {
        eprintln!("Произошла ошибка: {}", err);
    }
    Ok(())
}
