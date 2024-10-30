use std::sync::Arc;

use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use shared::SmsRequest;

use crate::error::AppResult;

#[derive(Clone)]
pub struct RabbitMQ {
    pub connection: Arc<Connection>,
    pub channel: Arc<Channel>,
}

impl RabbitMQ {
    pub async fn new(amqp_url: &str) -> AppResult<Self> {
        let connection = Connection::connect(amqp_url, ConnectionProperties::default()).await?;

        let channel = connection.create_channel().await?;

        channel
            .queue_declare(
                "sms_queue",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(RabbitMQ {
            connection: Arc::new(connection),
            channel: Arc::new(channel),
        })
    }

    pub async fn publish_message(&self, sms: SmsRequest) -> AppResult<()> {
        let payload = serde_json::to_vec(&sms)?;

        self.channel
            .basic_publish(
                "",
                "sms_queue",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }
}
