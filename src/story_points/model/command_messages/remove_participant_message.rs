use serde::{Deserialize, Serialize};
use uuid::{Uuid, serde::compact};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveParticipantMessage {
    #[serde(with = "compact")]
    participant_id: Uuid,
}