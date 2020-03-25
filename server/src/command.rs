use super::logger::Logger;
use super::redis_connector::RedisConnector;
use super::models::Card;
use serde_json::{to_string};

pub struct CardCommander {
    logger: Logger,
    redis_connector: RedisConnector,
}

impl CardCommander {
    pub fn new(logger: Logger, redis_connector: RedisConnector) -> Self {
        CardCommander {
            logger: logger,
            redis_connector: redis_connector,
        }
    }

    pub fn add(&self, card: &Card) -> Result<String, String> {
        let mut redis_connection = self.redis_connector.get_connection();

        let card_key = format!("{}::deck::{}::card::{}", card.user, card.deck, card.item);
        let serialized_card = to_string(card).unwrap();

        redis::cmd("SET").arg(card_key.clone()).arg(serialized_card).execute(&mut redis_connection);
        
        redis::cmd("SADD").arg(format!("{}::deck::{}", card.user, card.deck)).arg(card_key).execute(&mut redis_connection);

        Ok(String::from(""))
    }
}