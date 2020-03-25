use super::Logger;

/// Manages redis connections.
#[derive(Clone)]
pub struct RedisConnector {
    redis_client: redis::Client,
    logger: Logger
}

impl RedisConnector {
    /// Initializes a new instance.
    pub fn new(logger: Logger, redis_client: redis::Client) -> Self {
        RedisConnector {
            logger: logger,
            redis_client: redis_client,
        }
    }

    /// Creates a new redis connection.
    pub fn get_connection(&self) -> redis::Connection {
        self.redis_client.get_connection().expect("Failed to connect to redis DB.")
    }
}