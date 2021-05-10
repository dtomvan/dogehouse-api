use crate::endpoints::Endpoint;
use crate::queries::Query;

#[tokio::test]
async fn statistics() {
    crate::endpoints::Statistics::send().await.unwrap();
}

#[tokio::test]
async fn popular_rooms() {
    crate::endpoints::PopularRooms::send().await.unwrap();
}

#[tokio::test]
async fn scheduled_rooms() {
    crate::endpoints::ScheduledRooms::send().await.unwrap();
}

#[tokio::test]
async fn bots() {
    crate::endpoints::Bots::send().await.unwrap();
}

#[tokio::test]
async fn user_search() {
    crate::queries::UserByUsername::send("benawad").await.unwrap();
}
