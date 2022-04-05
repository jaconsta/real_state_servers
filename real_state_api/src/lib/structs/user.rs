use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub pss_hash: Option<String>
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl UserResponse {
    pub fn build_from_user(user: User) -> UserResponse {
        UserResponse{
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            phone: user.phone
        }
    }
}
