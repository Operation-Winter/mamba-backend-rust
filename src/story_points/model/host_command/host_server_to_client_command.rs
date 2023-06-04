use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
enum HostServerToClientCommand {
    // StartSession { uuid: Uuid, message: StartSessionMessage },
    // AddTicket(),
    // SkipVote(),
    // RemoveParticipant(),
    // EndSession(),
    // FinishVoting(),
    // Revote(),
    // Reconnect(),
    // EditTicket(),
    // AddTimer(),
    // CancelTimer(),
    // PreviousTickets(),
    // RequestCoffeeBreak(),
    // StartCoffeeBreakVote(),
    // EndCoffeeBreakVote()
}