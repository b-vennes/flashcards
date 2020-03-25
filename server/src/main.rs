mod logger;
mod command;
mod query;
mod redis_connector;
mod models;

use logger::Logger;
use models::{Card, CardQuery};
use command::CardCommander;
use query::CardRetriever;
use redis_connector::RedisConnector;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post};

#[get("/cards/query")]
async fn query_cards(
            query: web::Json<CardQuery>,
            card_retriever: web::Data<CardRetriever>,
            logger: web::Data<Logger>) -> impl Responder {
    match card_retriever.query(&query) {
        Ok(cards) => HttpResponse::Ok().json(cards),
        Err(error) => {
            logger.log_error(format!("Error during card query: {}", error));
            HttpResponse::InternalServerError().body(error)
        }
    }
}

#[post("/cards")]
async fn add_card(
            card: web::Json<Card>,
            card_commander: web::Data<CardCommander>, 
            logger: web::Data<Logger>) -> impl Responder {
    match card_commander.add(&card) {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(error) => {
            logger.log_error(format!("Error during add card command: {}", error));
            HttpResponse::InternalServerError().body(error)
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let logger = Logger {};
        let redis_client = redis::Client::open("redis://127.0.0.1:6379").expect("Unable to open redis client.");

        let redis_connector = RedisConnector::new(logger.clone(), redis_client.clone());
        
        let card_commander = CardCommander::new(logger.clone(), redis_connector.clone());
        let card_retriever = CardRetriever::new(logger.clone(), redis_connector.clone());

        App::new()
            .service(query_cards)
            .service(add_card)
            .data(card_commander)
            .data(card_retriever)
            .data(logger.clone())
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
