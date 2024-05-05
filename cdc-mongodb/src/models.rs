use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub value: i32,
}
