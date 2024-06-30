use crate::{
    events::{event_handler::Handler, models::event::Event},
    manger_container_structs::RabbitMQMangerContainer,
};
use lapin::{options::BasicPublishOptions, BasicProperties};
use serde::{Deserialize, Serialize};
use serenity::model::prelude::MessageUpdateEvent;
use serenity::prelude::Context;
use tracing::debug;

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEvent {
    pub event: Event,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: MessageUpdateEvent,
}

impl Handler {
    pub async fn handle_message_update(&self, ctx: Context, event: MessageUpdateEvent) {
        let data = ctx.clone();
        let data_read = data.data.read().await;

        let channel = data_read
            .get::<RabbitMQMangerContainer>()
            .expect("[EVENT/MESSAGE_UPDATE] failed to get the Rabbit MQ Channel");

        let event = MessageDeleteEvent {
            event: Event::MessageUpdate,
            shard_id: ctx.shard_id.0,
            timestamp: Handler::get_unix_time(),
            content: event,
        };
        let serlized = serde_json::to_string(&event)
            .expect("[EVENT/MESSAGE_UPDATE] failed to serialize event");

        let payload = serlized.as_bytes();

        let confirm = channel
            .basic_publish(
                "",
                "bulbbot.gateway",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .expect("[EVENT/MESSAGE_UPDATE] failed to publish to channel")
            .await
            .expect("[EVENT/MESSAGE_UPDATE] failed to get confirmation message from channel");

        debug!("Rabbit MQ channel publish return message: {:#?}", confirm);
    }
}
