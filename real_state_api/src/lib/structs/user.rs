use mongodb::bson::oid::ObjectId;
// use mongodb::bson::serde_helpers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] //  , serialize_with = "serde_helpers::serialize_object_id_as_hex_string")]
    pub id: Option<ObjectId>,
    pub name: String
}