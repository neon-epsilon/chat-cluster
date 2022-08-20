use std::convert::Infallible;

use server::chat_client::ChatClient;
use tokio;
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let chat_client = ChatClient::new();

    let health_route = warp::path!("health").and_then(health_handler);
    let messages_route = warp::path!("messages")
        .and(with_chat_client(chat_client))
        .and_then(messages_handler);

    let routes = health_route.or(messages_route);

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_chat_client(
    client: ChatClient,
) -> impl Filter<Extract = (ChatClient,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

async fn health_handler() -> Result<impl Reply, Infallible> {
    Ok("OK")
}

async fn messages_handler(chat_client: ChatClient) -> Result<impl Reply, Infallible> {
    let serialized_messages = format!("{:?}", chat_client.messages_received());

    Ok(serialized_messages)
}
