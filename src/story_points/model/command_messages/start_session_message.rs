use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::story_points::model::system::card::Card;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartSessionMessage {
    pub session_name: String,
    pub auto_complete_voting: bool,
    pub available_cards: Vec<Card>,
    pub password: Option<String>,
    pub tags: HashSet<String>,
}