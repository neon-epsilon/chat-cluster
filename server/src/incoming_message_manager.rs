use redis::{Msg, RedisError};
use thiserror::Error;
use warp::Stream;

pub struct IncomingMessageManager {
    redis_url: String,
}

impl IncomingMessageManager {
    pub fn new(redis_url: String) -> IncomingMessageManagerResult<Self> {
        Ok(IncomingMessageManager { redis_url })
    }

    //TODO: Stream Item should be our own `Msg` type, not `redis::Msg`.
    pub async fn subscribe(
        &mut self,
        channel_name: &str,
    ) -> IncomingMessageManagerResult<impl Stream<Item = Msg>> {
        let redis_client = redis::Client::open(self.redis_url.clone())?;
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
