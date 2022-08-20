use redis::{Msg, RedisError};
use thiserror::Error;
use warp::Stream;

static REDIS_CONNECTION_STRING: &str = "redis://127.0.0.1:6379";
pub struct IncomingMessageManager {}

impl IncomingMessageManager {
    //TODO: Make redis connection string a parameter here.
    pub fn new() -> IncomingMessageManagerResult<Self> {
        Ok(IncomingMessageManager {})
    }

    //TODO: Stream Item should be our own `Msg` type, not `redis::Msg`.
    pub async fn subscribe(
        &mut self,
        channel_name: &str,
    ) -> IncomingMessageManagerResult<impl Stream<Item = Msg>> {
        let redis_client = redis::Client::open(REDIS_CONNECTION_STRING)?;
        let connection = redis_client.get_async_connection().await?;
        let mut pubsub = connection.into_pubsub();

        pubsub.subscribe(channel_name).await?;

        let stream = pubsub.into_on_message();

        Ok(stream)
    }
}

#[derive(Error, Debug)]
pub enum IncomingMessageManagerError {
    #[error(transparent)]
    Redis(#[from] RedisError),
}

pub type IncomingMessageManagerResult<T> = Result<T, IncomingMessageManagerError>;
