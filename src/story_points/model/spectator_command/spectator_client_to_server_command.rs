use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::story_points::model::command_messages::spectate_session_message::SpectateSessionMessage;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpectatorClientToServerCommand {
    JoinSession {
        uuid: Uuid,
        message: SpectateSessionMessage,
    },
    LeaveSession {
        uuid: Uuid,
    },
    Reconnect {
        uuid: Uuid,
    },
}
