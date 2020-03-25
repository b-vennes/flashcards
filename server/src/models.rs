use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub user: String,
    pub deck: String,
    pub item: String,
    pub definition: String,
}

#[derive(Serialize, Deserialize)]
pub struct CardQuery {
    pub user: String,
    pub deck: String,
    pub page_number: usize,
    pub page_size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct DeckView {
    pub cards: Vec<CardView>,
}

#[derive(Serialize, Deserialize)]
pub struct CardView {
    pub item: String,
    pub definition: String,
}