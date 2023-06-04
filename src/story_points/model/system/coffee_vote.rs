use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoffeeVote {
    participant_id: Uuid,
    vote: bool,
}