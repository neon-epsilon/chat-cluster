use std::convert::Infallible;

use anyhow::Result;
use common::ChatMessageStream;
use futures::StreamExt;
use replication_log::message_log::MessageLog;
use tokio;
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let all_channels_stream = subscribe_all_channels("redis://message-broker-service:6379")
        .await
        .unwrap();

    let message_log = MessageLog::new(all_channels_stream);

    let messages_route = warp::path!("messages" / String)
        .and(with_message_log(message_log))
        .and_then(messages_handler);

    let routes = messages_route;

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_message_log(
    message_log: MessageLog,
) -> impl Filter<Extract = (MessageLog,), Error = Infallible> + Clone {
    warp::any().map(move || message_log.clone())
}

async fn messages_handler(
    channel_name: String,
    message_log: MessageLog,
) -> Result<impl Reply, Infallible> {
    match serde_json::to_string(&message_log.messages_received(&channel_name)) {
        Ok(serialized_messages) => Ok(warp::reply::with_status(
            serialized_messages,
            warp::http::StatusCode::OK,
        )),
        Err(err) => Ok(warp::reply::with_status(
            err.to_string(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

async fn subscribe_all_channels(redis_url: &str) -> Result<ChatMessageStream> {
    let redis_client = redis::Client::open(redis_url)?;
    let connection = redis_client.get_async_connection().await?;
    let mut pubsub = connection.into_pubsub();

    pubsub.psubscribe("*").await?;

    let stream = pubsub
        .into_on_message()
        .map(common::chat_message_from_redis_msg);

    Ok(Box::pin(stream))
}
