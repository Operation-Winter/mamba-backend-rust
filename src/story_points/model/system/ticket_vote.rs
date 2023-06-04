use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::card::Card;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TicketVote {
    participant_id: Uuid,
    selected_card: Card,
    tag: Option<String>,
}