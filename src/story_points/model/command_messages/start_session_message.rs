use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::story_points::model::system::card::Card;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartSessionMessage {
    session_name: String,
    auto_complete_voting: bool,
    available_cards: Vec<Card>,
    password: Option<String>,
    tags: HashSet<String>,
}