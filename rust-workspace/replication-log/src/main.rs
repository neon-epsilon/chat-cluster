use std::convert::Infallible;

use anyhow::Result;
use futures::StreamExt;
use redis::Msg;
use replication_log::{message_log::MessageLog, ChatMessage, MessageStream};
use tokio;
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    //TODO: make the redis url configurable via env vars or config file.
    let all_channels_stream = subscribe_all_channels("redis://localhost:6379")
        .await
        .unwrap();

    let message_log = MessageLog::new(all_channels_stream);

    let messages_route = warp::path!("messages")
        .and(with_message_log(message_log))
        .and_then(messages_handler);

    let routes = messages_route;

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_message_log(
    message_log: MessageLog,
) -> impl Filter<Extract = (MessageLog,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || message_log.clone())
}

async fn messages_handler(message_log: MessageLog) -> Result<impl Reply, Infallible> {
    let serialized_messages = format!("{:?}", message_log.messages_received());

    Ok(serialized_messages)
}

async fn subscribe_all_channels(redis_url: &str) -> Result<MessageStream> {
    let redis_client = redis::Client::open(redis_url)?;
    let connection = redis_client.get_async_connection().await?;
    let mut pubsub = connection.into_pubsub();

    pubsub.psubscribe("*").await?;

    let stream = pubsub.into_on_message().map(chat_message_from_redis_msg);

    Ok(Box::pin(stream))
}

//TODO: refactor into common crate
fn chat_message_from_redis_msg(msg: Msg) -> Result<ChatMessage> {
    Ok(ChatMessage {
        channel: msg.get_channel()?,
        message_text: msg.get_payload()?,
    })
}