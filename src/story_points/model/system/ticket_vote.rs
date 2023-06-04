use serde::{Deserialize, Serialize};

use super::card::Card;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketVote {
    participant_id: String,
    selected_card: Card,
    tag: Option<String>,
}