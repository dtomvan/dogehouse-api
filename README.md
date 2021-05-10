# dogehouse-api

## Dogehouse-api
This is a work-in-progress crate to interact with the dogehouse API.
Note that the API's documentation isn't complete yet, and so is the API itself.
This uses reqwest and serde-json. I don't know if libraries should use something different from
reqwest. If so, please file an issue on GitHub.
### Current features:
- Get a list of popular rooms
- Get a list of upcoming rooms
- Get a list of bots
- Get basic statistics
- Get individual users by username.
### Caveats:
- The API is changing by the minute at the current stage, and this can break this crate.
- JavaScript and by that JSON uses 64-bit numbers, so all numbers in this crate have to be
either u64's or i64's
### Usage:
This crate consists of Endpoint's and Query's.
Endpoints are just the simple API endpoints, which does not take any arguments.
Queries are API endpoints, which take one or more arguments, such as "/search/some_username".
