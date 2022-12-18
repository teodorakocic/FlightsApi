use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub date: String,
    pub city_code: String,
    pub precipitation: f32,
    pub temperature_max: f32,
    pub temperature_min: f32,
    pub wind: f32,
    pub weather: String,
}