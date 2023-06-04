use serde::{Deserialize, Serialize};
use uuid::Uuid;
use uuid::serde::compact;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkipVoteMessage {
    #[serde(with = "compact")]
    participant_id: Uuid,
}