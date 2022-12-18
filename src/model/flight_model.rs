use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Flight {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub origin: String,
    pub destination: String,
    pub departure_date: String,
    pub return_date: String,
    pub adults: i32,
    pub children: i32,
    pub infants: i32,
    pub travel_class: String,
    pub non_stop: bool,
    pub currency_code: String,
    pub max_price: f32,
    pub max_offers: i32,
}