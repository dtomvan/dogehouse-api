use serde::Deserialize;

/// Defines a query, which takes one or more arguments.
#[async_trait::async_trait]
pub trait Query {
    /// Defines the scheme that the API shall return.
    type Scheme;
    /// Defines which args will be passed into the API.
    type Args;
    /// Sends the query to the API.
    async fn send(_: Self::Args) -> Result<Self::Scheme, crate::ErrorType>;
}

/// Query users by username. The passed-in arg will be a username.
pub struct UserByUsername;
#[async_trait::async_trait]
impl Query for UserByUsername {
    type Scheme = UsersScheme;
    /// The username you are searching for.
    type Args = &'static str;
    async fn send(username: Self::Args) -> Result<Self::Scheme, crate::ErrorType> {
        let url = format!("{}/search/{}", crate::endpoints::BASE_URL, username);
        crate::endpoint!(UsersScheme,url)
    }
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Defines all users which are caught by the [UserByUsername] request.
pub struct UsersScheme {
    items: Vec<crate::endpoints::User>,
}
