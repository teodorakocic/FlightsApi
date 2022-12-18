use std::cmp::Ordering;

use crate::{model::flight_model::Flight, repository::database::MongoRepo};
use mongodb::{results::InsertOneResult, bson::oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};

#[post("/flight", data = "<new_flight>")]
pub fn create_flight(
    db: &State<MongoRepo>,
    mut new_flight: Json<Flight>,
) -> Result<Json<InsertOneResult>, Status> {
    let travel_class = new_flight.travel_class.to_lowercase();
    if travel_class != "economy" && travel_class != "firt" && travel_class != "business" && travel_class != "premium"
    {
        new_flight.travel_class = "Economy".to_owned();
    }
    if new_flight.return_date.to_owned().cmp(&new_flight.departure_date.to_owned()) == Ordering::Less {
        new_flight.return_date = new_flight.departure_date.to_owned()
    }
    let data = Flight {
        id: None,
        origin: new_flight.origin.to_owned(),
        destination: new_flight.destination.to_owned(),
        departure_date: new_flight.departure_date.to_owned(),
        return_date: new_flight.return_date.to_owned(),
        adults: new_flight.adults.to_owned(),
        children: new_flight.children.to_owned(),
        infants: new_flight.infants.to_owned(),
        travel_class: new_flight.travel_class.to_owned(),
        non_stop: new_flight.non_stop.to_owned(),
        currency_code: new_flight.currency_code.to_owned(),
        max_price: new_flight.max_price.to_owned(),
        max_offers: new_flight.max_offers.to_owned(),
    };
    let flight_detail = db.create_flight(data);
    match flight_detail {
        Ok(flight) => Ok(Json(flight)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flight/<id>")]
pub fn get_flight(db: &State<MongoRepo>, id: String) -> Result<Json<Flight>, Status> {
    let id_db = id;
    if id_db.is_empty() {
        return Err(Status::BadRequest);
    };
    let flight_detail = db.get_flight(&id_db);
    match flight_detail {
        Ok(flight) => Ok(Json(flight)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flights/<destination>")]
pub fn get_flights_by_destination(db: &State<MongoRepo>, destination: String) -> Result<Json<Vec<Flight>>, Status> {
    if destination.is_empty() || destination.len() != 3 {
        return Err(Status::BadRequest);
    };
    let flights = db.get_flights_by_destination(&destination);
    match flights {
        Ok(fligths) => Ok(Json(fligths)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flights/<origin>/<date>", rank = 2)]
pub fn get_flights_by_origin_date(db: &State<MongoRepo>, origin: String, date: String) -> Result<Json<Vec<Flight>>, Status> {
    if origin.is_empty() || date.is_empty() || origin.len() != 3 {
        return Err(Status::BadRequest);
    };
    let flights = db.get_flights_by_origin_date(&origin, &date);
    match flights {
        Ok(fligths) => Ok(Json(fligths)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flights/<max_price>/<min_price>/<currency>", rank = 2)]
pub fn get_flights_in_price_range(db: &State<MongoRepo>, max_price: f32, min_price: f32, currency: String) -> Result<Json<Vec<Flight>>, Status> {
    if (max_price < min_price || max_price == 0.0) && currency.is_empty() {
        return Err(Status::BadRequest);
    };
    let flights = db.get_flights_in_price_range(max_price, min_price, &currency);
    match flights {
        Ok(fligths) => Ok(Json(fligths)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flights/<destination>/<stop>", rank = 1)]
pub fn get_flights_with_connection_for_destination(db: &State<MongoRepo>, destination: String, stop: bool) -> Result<Json<Vec<Flight>>, Status> {
    if destination.is_empty() {
        return Err(Status::BadRequest);
    };
    let flights = db.get_flights_with_connection_for_destination(&destination, stop);
    match flights {
        Ok(fligths) => Ok(Json(fligths)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flights/<adults>/<children>/<infants>", rank = 1)]
pub fn get_flights_with_passengers_numbers(db: &State<MongoRepo>, adults: i32, children: i32, infants: i32) -> Result<Json<Vec<Flight>>, Status> {
    let flights = db.get_flights_with_passengers_numbers(adults, children, infants);
    match flights {
        Ok(fligths) => Ok(Json(fligths)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flights")]
pub fn get_all_flights(db: &State<MongoRepo>) -> Result<Json<Vec<Flight>>, Status> {
    let flights = db.get_all_flights();
    match flights {
        Ok(flights) => Ok(Json(flights)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/flights/<id>", data = "<new_flight>")]
pub fn update_flight_date_travel_class(db: &State<MongoRepo>, id: String, new_flight: Json<Flight>,) -> Result<Json<Flight>, Status> {
    let id_db = id;
    if id_db.is_empty() || new_flight.max_offers < 10 {
        return Err(Status::BadRequest);
    };
    let data = Flight {
        id: Some(ObjectId::parse_str(&id_db).unwrap()),
        origin: new_flight.origin.to_owned(),
        destination: new_flight.destination.to_owned(),
        departure_date: new_flight.departure_date.to_owned(),
        return_date: new_flight.return_date.to_owned(),
        adults: new_flight.adults.to_owned(),
        children: new_flight.children.to_owned(),
        infants: new_flight.infants.to_owned(),
        travel_class: new_flight.travel_class.to_owned(),
        non_stop: new_flight.non_stop.to_owned(),
        currency_code: new_flight.currency_code.to_owned(),
        max_price: new_flight.max_price.to_owned(),
        max_offers: new_flight.max_offers.to_owned(),
    };
    let update_result = db.update_flight_date_travel_class(&id_db, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_flight_info = db.get_flight(&id_db);
                return match updated_flight_info {
                    Ok(flight) => Ok(Json(flight)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/flight/<id>")]
pub fn delete_flight(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    let id_db = id;
    if id_db.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_flight(&id_db);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Flight successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/flights")]
pub fn delete_all_flights(db: &State<MongoRepo>) -> Result<Json<&str>, Status> {
    let result = db.delete_all_flights();
    match result {
        Ok(res) => {
            if res.deleted_count > 0 {
                return Ok(Json("Flights successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
