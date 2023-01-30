use std::{convert::Infallible, sync::Arc};

use chat_server::{
    channel_subscriber::RedisChannelSubscriber, chat_server::ChatServer,
    replication_log_client::ReqwestReplicationLogClient,
};
use common::DEFAULT_CHANNEL;
use tokio;
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let channel_subscriber = RedisChannelSubscriber {
        redis_url: "redis://message-broker-service:6379".to_string(),
    };
    let replication_log_client = ReqwestReplicationLogClient {
        replication_log_url: "http://replication-log-service:80/messages".to_string(),
    };

    let chat_server = ChatServer::new(
        Arc::new(channel_subscriber),
        Arc::new(replication_log_client),
    );
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
