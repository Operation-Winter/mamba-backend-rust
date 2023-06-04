use serde::{Deserialize, Serialize};

use crate::story_points::model::command_messages::{
    invalid_command_message::InvalidCommandMessage, state_message::StateMessage,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
enum ParticipantServerToClientCommand {
    NoneState { message: StateMessage },
    VotingState { message: StateMessage },
    FinishedState { message: StateMessage },
    CoffeeVoting { message: StateMessage },
    CoffeeVotingFinished { message: StateMessage },
    InvalidCommand { message: InvalidCommandMessage },
    InvalidSession,
    EndSession,
    SessionIdleTimeout,
    RemoveParticipant,
}
