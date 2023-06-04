use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::story_points::model::command_messages::{
    add_timer_message::AddTimerMessage, remove_participant_message::RemoveParticipantMessage,
    skip_vote_message::SkipVoteMessage, start_session_message::StartSessionMessage,
    ticket_message::TicketMessage,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HostClientToServerCommand {
    StartSession {
        uuid: Uuid,
        message: StartSessionMessage,
    },
    EndSession {
        uuid: Uuid,
    },
    Reconnect {
        uuid: Uuid,
    },
    AddTicket {
        uuid: Uuid,
        message: TicketMessage,
    },
    EditTicket {
        uuid: Uuid,
        message: TicketMessage,
    },
    PreviousTickets {
        uuid: Uuid,
    },
    SkipVote {
        uuid: Uuid,
        message: SkipVoteMessage,
    },
    FinishVoting {
        uuid: Uuid,
    },
    Revote {
        uuid: Uuid,
    },
    RemoveParticipant {
        uuid: Uuid,
        message: RemoveParticipantMessage,
    },
    AddTimer {
        uuid: Uuid,
        message: AddTimerMessage,
    },
    CancelTimer {
        uuid: Uuid,
    },
    RequestCoffeeBreak {
        uuid: Uuid,
    },
    StartCoffeeBreakVote {
        uuid: Uuid,
    },
    EndCoffeeBreakVote {
        uuid: Uuid,
    },
}

#[cfg(test)]
#[path = "./host_client_to_server_command_test.rs"]
mod host_client_to_server_command_test;
