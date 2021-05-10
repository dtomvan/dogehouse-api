use std::collections::BTreeMap as Map;
use serde::Deserialize;
use serde_json::Value;

pub(crate) const BASE_URL: &'static str = "https://api.dogegarden.net/v1";

#[async_trait::async_trait]
/// Defines a 
pub trait Endpoint {
    /// Defines which scheme the endpoint is using.
    type Scheme;
    /// Sends a request to the endpoint, and deserializes it.
    async fn send() -> Result<Self::Scheme, crate::ErrorType>;
}

crate::endpoint!(
    #[doc = "This defines the binding for the /statistics endpoint in the API."]
    Statistics,
    StatisticsScheme,
    "/statistics"
);

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// The scheme defining what the API will return upon utilizing the /statistics endpoint.
pub struct StatisticsScheme {
    total_rooms: u64,
    total_scheduled_rooms: u64,
    total_registered: u64,
    total_online: u64,
    total_bots_online: u64,
    total_bots_sending_telemetry: u64,
}

crate::endpoint!(
    #[doc = "This defines the binding for the /popularRooms endpoint in the API."]
    PopularRooms,
    PopularRoomsScheme,
    "/popularRooms"
);

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// The scheme defining what the API will return upon utilizing the /popularRooms endpoint.
pub struct PopularRoomsScheme {
    rooms: Vec<PopularRoom>,
    #[serde(flatten)]
    // Some undocumented items are inside of /popularRooms
    undocumented: Map<String, Value>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// A room in a [PopularRoomsScheme].
pub struct PopularRoom {
    id: String,
    name: String,
    description: String,
    num_people_inside: u64,
    is_private: bool,
    // Undocumented
    chat_mode: String,
    // Undocumented
    chat_throttle: u64,
    // Undocumented
    auto_speaker: bool,
    creator_id: String,
    people_preview_list: Vec<PopularRoomUser>,
    voice_server_id: String,
    // Really Dogegarden?
    #[serde(rename = "inserted_at")]
    inserted_at: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Defines a user inside of a [PopularRoom].
pub struct PopularRoomUser {
    id: String,
    display_name: String,
    num_followers: u64,
    avatar_url: String,
}

crate::endpoint!(
    #[doc = "This defines the binding for the /bots endpoint in the API."]
    Bots,
    Vec<Bot>,
    "/bots"
);

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Defines a bot in the /bots API endpoint.
pub struct Bot {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "socket_id")]
    socket_id: String,
    bot: BotInner,
    // This isn't even the same as other rooms
    // Or in the docs
    room: Map<String, Value>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// For some reason, a bot contains a bot field, which contains these fields.
pub struct BotInner {
    uuid: String,
    username: String,
    avatar: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Defines a room, which a [Bot] is in.
pub struct BotRoom {
    // Not in docs, AFAIK always null.
    uuid: Value,
    name: String,
    listening: u64,
    // Not in the docs, AFAIK always an empty vec.
    users: Vec<Value>,
}

crate::endpoint!(
    #[doc = "This defines the binding for the /scheduledRooms endpoint in the API."]
    ScheduledRooms,
    ScheduledRoomsScheme,
    "/scheduledRooms"
);

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// The scheme defining what the API will return upon utilizing the /scheduledRooms endpoint.
pub struct ScheduledRoomsScheme {
    scheduled_rooms: Vec<ScheduledRoom>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// A room in the [ScheduledRoomsScheme].
pub struct ScheduledRoom {
    id: String,
    name: String,
    num_attending: u64,
    scheduled_for: String,
    description: String,
    // ?? This will always be null, since the room hasn't started yet; It's an upcoming room.
    room_id: Value,
    creator: User,
    creator_id: String,
}


#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// The owner of a [ScheduledRoom], or a user queried after calling
/// [crate::queries::UserByUserName].
pub struct User {
    avatar_url: Option<String>,
    banner_url: Option<String>,
    bio: String,
    // Not documented, and I cannot make sense out of this.
    bot_owner_id: Value,
    display_name: String,
    // In the current state of the API always null.
    follows_you: Value,
    // In the current state of the API always null.
    i_blocked_them: Value,
    id: String,
    last_online: String,
    num_followers: u64,
    num_following: u64,
    online: bool,
    // In the current state of the API always null.
    room_permissions: Value,
    username: String,
    // "on" or "off" ??
    whisper_privacy_setting: String,
    // In the current state of the API always null.
    you_are_following: Value,
}
