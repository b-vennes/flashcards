use super::models::{CardQuery, Card, CardView, DeckView};
use super::logger::Logger;
use super::redis_connector::RedisConnector;
use serde_json::{from_str};
use redis::RedisError;

pub struct CardRetriever {
    logger: Logger,
    redis_connector: RedisConnector,
}

impl CardRetriever {
    pub fn new(logger: Logger, redis_connector: RedisConnector) -> Self {
        CardRetriever {
            logger: logger,
            redis_connector: redis_connector,
        }
    }

    pub fn query(&self, query: &CardQuery) -> Result<DeckView, String> {
        let mut redis_connection = self.redis_connector.get_connection();

        let query_result: Result<Vec<String>, RedisError> = redis::cmd("SMEMBERS")
            .arg(format!("{}::deck::{}", query.user, query.deck))
            .query(&mut redis_connection);
        
        let cards: Vec<String> = match query_result {
                Ok(cards) => cards,
                _ => return Err(String::from("Error with SMEMBERS query."))
            };

        let start = query.page_number * query.page_size;
        let mut end = start + query.page_size;
        let cards_length = cards.len();

        if cards_length < start {
            return Err(String::from("Page not found."));
        }
        else if cards_length < end {
            end = cards_length;
        }

        let card_views: Vec<CardView> = cards[start..end]
            .iter()
            .map(|card_key: &String| {
                redis::cmd("GET")
                    .arg(card_key)
                    .query(&mut redis_connection)
                    .expect("Error during card value query.")
            })
            .map(|card_value: String| {
                let card: Card = from_str(&card_value).unwrap();
                CardView {
                    item: card.item.clone(),
                    definition: card.definition.clone(),
                }
            })
            .collect();

        Ok(DeckView {
            cards: card_views
        })
    }
}