use ::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub user_uuid: String,
    pub username: String,
    pub password: String,
    pub location: String,
    pub major: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub location: String,
    pub major: String,
}

impl User {
    pub fn new(
        user_uuid: String,
        username: String,
        password: String,
        location: String,
        major: String,
    ) -> User {
        User {
            user_uuid,
            username,
            password,
            location,
            major,
        }
    }
}
