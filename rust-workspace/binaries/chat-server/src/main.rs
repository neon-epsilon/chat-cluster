use std::{convert::Infallible, sync::Arc};

use chat_server::{channel_subscriber::RedisChannelSubscriber, chat_server::ChatServer};
use common::DEFAULT_CHANNEL;
use tokio;
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    // TODO: make the redis url configurable via env vars or config file.
    let channel_subscriber =
        RedisChannelSubscriber::new("redis://message-broker-service:6379".to_string());

    let chat_server = ChatServer::new(Arc::new(channel_subscriber));
    // TODO: connect to the replication log on startup and hand access to it to the chat server.
    chat_server.subscribe(DEFAULT_CHANNEL).await.unwrap();

    let messages_route = warp::path!("messages")
        .and(with_chat_server(chat_server))
        .and_then(messages_handler);

    let routes = messages_route;

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_chat_server(
    server: ChatServer,
) -> impl Filter<Extract = (ChatServer,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || server.clone())
}

async fn messages_handler(chat_server: ChatServer) -> Result<impl Reply, Infallible> {
    let serialized_messages = format!("{:?}", chat_server.messages_received());

    Ok(serialized_messages)
}
