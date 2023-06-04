use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::story_points::model::command_messages::{
    change_name_message::ChangeNameMessage, coffee_break_vote_message::CoffeeBreakVoteMessage,
    join_session_message::JoinSessionMessage, vote_message::VoteMessage,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
enum ParticipantClientToServerCommand {
    JoinSession {
        uuid: Uuid,
        message: JoinSessionMessage,
    },
    Vote {
        uuid: Uuid,
        message: VoteMessage,
    },
    LeaveSession {
        uuid: Uuid,
    },
    Reconnect {
        uuid: Uuid,
    },
    ChangeName {
        uuid: Uuid,
        message: ChangeNameMessage,
    },
    RequestCoffeeBreak {
        uuid: Uuid,
    },
    CoffeeBreakVote {
        uuid: Uuid,
        message: CoffeeBreakVoteMessage,
    },
}
