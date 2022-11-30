use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserPostRequest {
    user_name: String,
    password: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClassIdentifier {
    class_id: String,
}
