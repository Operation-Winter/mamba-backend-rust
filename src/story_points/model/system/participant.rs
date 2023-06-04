use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    participant_id: Uuid,
    name: String,
    connected: bool,
}