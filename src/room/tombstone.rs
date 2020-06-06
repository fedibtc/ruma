//! Types for the *m.room.tombstone* event.

use ruma_events_macros::StateEventContent;
use ruma_identifiers::RoomId;
use serde::{Deserialize, Serialize};

/// A state event signifying that a room has been upgraded to a different room version, and that
/// clients should go there.
#[derive(Clone, Debug, Deserialize, Serialize, StateEventContent)]
#[ruma_event(type = "m.room.tombstone")]
pub struct TombstoneEventContent {
    /// A server-defined message.
    pub body: String,

    /// The new room the client should be visiting.
    pub replacement_room: RoomId,
}
