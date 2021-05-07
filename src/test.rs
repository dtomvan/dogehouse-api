use crate::endpoints::Endpoint;

#[tokio::test]
async fn statistics() {
    crate::endpoints::Statistics::send().await.unwrap();
}

#[tokio::test]
async fn popular_rooms() {
    crate::endpoints::PopularRooms::send().await.unwrap();
}
