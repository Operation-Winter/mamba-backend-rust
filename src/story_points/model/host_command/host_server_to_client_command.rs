use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::story_points::model::command_messages::{
    invalid_command_message::InvalidCommandMessage,
    previous_tickets_message::PreviousTicketsMessage, state_message::StateMessage,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
enum HostServerToClientCommand {
    NoneState {
        uuid: Uuid,
        message: StateMessage,
    },
    VotingState {
        uuid: Uuid,
        message: StateMessage,
    },
    FinishedState {
        uuid: Uuid,
        message: StateMessage,
    },
    CoffeeVoting {
        uuid: Uuid,
        message: StateMessage,
    },
    CoffeeVotingFinished {
        uuid: Uuid,
        message: StateMessage,
    },
    InvalidCommand {
        uuid: Uuid,
        message: InvalidCommandMessage,
    },
    PreviousTickets {
        uuid: Uuid,
        message: PreviousTicketsMessage,
    },
    SessionIdleTimeout,
}
