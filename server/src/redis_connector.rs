use super::Logger;

#[derive(Clone)]
pub struct RedisConnector {
    redis_client: redis::Client,
    logger: Logger
}

impl RedisConnector {
    pub fn new(logger: Logger, redis_client: redis::Client) -> Self {
        RedisConnector {
            logger: logger,
            redis_client: redis_client,
        }
    }

    pub fn get_connection(&self) -> redis::Connection {
        self.redis_client.get_connection().expect("Failed to connect to redis DB.")
    }
}