use std::collections::BTreeMap as Map;
use serde::Deserialize;
use serde_json::Value;

// u64's are being used since these are around the limits of JavaScript.
// String's are being used, because serde's derive macro currently does not support &'static str
// very well.

const BASE_URL: &'static str = "https://api.dogegarden.net/v1";

#[async_trait::async_trait]
pub trait Endpoint {
    type Scheme;
    async fn send() -> Result<Self::Scheme, crate::ErrorType>;
}

crate::endpoint!(
    #[doc = "This defines the binding for the /statistics endpoint in the API."]
    Statistics,
    StatisticsScheme,
    "/statistics"
);
#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PopularRoomsScheme {
    rooms: Vec<RoomScheme>,
    #[serde(flatten)]
    // Some undocumented items are inside of /popularRooms
    undocumented: Map<String, Value>,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoomScheme {
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
    people_preview_list: Vec<PersonScheme>,
    voice_server_id: String,
    // Really Dogegarden?
    #[serde(rename = "inserted_at")]
    inserted_at: String,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PersonScheme {
    id: String,
    display_name: String,
    num_followers: u64,
    avatar_url: String,
}

crate::endpoint!(
    #[doc = "This defines the binding for the /bots endpoint in the API."]
    Bots,
    Vec<BotScheme>,
    "/bots"
);

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BotScheme {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "socket_id")]
    socket_id: String,
    bot: BotInner,
    // This isn't even the same as other rooms
    // Or in the docs
    room: Map<String, Value>,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BotInner {
    uuid: String,
    username: String,
    avatar: String,
}
