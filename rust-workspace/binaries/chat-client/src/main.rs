use std::{convert::Infallible, sync::Arc};

use chat_client::{channel_subscriber::RedisChannelSubscriber, chat_client::ChatClient};
use common::DEFAULT_CHANNEL;
use tokio;
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    //TODO: make the redis url configurable via env vars or config file.
    let channel_subscriber =
        RedisChannelSubscriber::new("redis://message-broker-service:6379".to_string());

    let chat_client = ChatClient::new(Arc::new(channel_subscriber));
    chat_client.subscribe(DEFAULT_CHANNEL).await.unwrap();

    let messages_route = warp::path!("messages")
        .and(with_chat_client(chat_client))
        .and_then(messages_handler);

    let routes = messages_route;

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_chat_client(
    client: ChatClient,
) -> impl Filter<Extract = (ChatClient,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

async fn messages_handler(chat_client: ChatClient) -> Result<impl Reply, Infallible> {
    let serialized_messages = format!("{:?}", chat_client.messages_received());

    Ok(serialized_messages)
}
