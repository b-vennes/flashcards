use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Deck {
    key: String,
    value: String
}

#[get("/decks/{deck}")]
async fn get_deck(path: web::Path<(String,)>, data_access: web::Data<DataAccess>) -> impl Responder {
    let decks = data_access.get_deck(&path.0);
    HttpResponse::Ok().json(decks)
}

#[post("/decks")]
async fn set_deck(deck: web::Json<Deck>, logger: web::Data<Logger>, data_access: web::Data<DataAccess>) -> impl Responder {
    data_access.set_deck(&deck.key, &deck.value);
    HttpResponse::Ok()
}

struct DataAccess {
    redis_client: redis::Client,
    logger: Logger
}

impl DataAccess {
    fn new(logger: Logger, redis_client: redis::Client) -> Self {
        DataAccess {
            logger: logger,
            redis_client: redis_client,
        }
    }

    fn get_deck(&self, deck_key: &String) -> String {
        let mut redis_connection = self.get_connection();
        
        redis::cmd("GET").arg(deck_key).query(&mut redis_connection).expect("Errored during query")
    }

    fn set_deck(&self, deck_key: &String, deck_value: &String) {
        let mut redis_connection = self.get_connection();

        redis::cmd("SET").arg(deck_key).arg(deck_value).execute(&mut redis_connection);
    }

    fn get_connection(&self) -> redis::Connection {
        self.redis_client.get_connection().expect("Failed to get redis connection.")
    }
}

struct Logger {}

impl Logger {
    fn log_info(message: &String) {
        println!("{}", message);
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let redis_client = redis::Client::open("redis://127.0.0.1:6379").expect("Unable to open redis client.");
        let logger = Logger {};

        let data_access = DataAccess::new(logger, redis_client);

        App::new()
            .service(get_deck)
            .service(set_deck)
            .data(data_access)
            .data(Logger {})
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
