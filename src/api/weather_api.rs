use crate::{model::{weather_model::Weather, flight_model::Flight}, repository::database::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/weather", data = "<new_report>")]
pub fn create_weather_report(
    db: &State<MongoRepo>,
    mut new_report: Json<Weather>,
) -> Result<Json<InsertOneResult>, Status> {
    if new_report.temperature_max < new_report.temperature_min {
        new_report.temperature_max = new_report.temperature_min;
    }
    let data = Weather {
        id: None,
        date: new_report.date.to_owned(),
        city_code: new_report.city_code.to_owned(),
        precipitation: new_report.precipitation.to_owned(),
        temperature_max: new_report.temperature_max.to_owned(),
        temperature_min: new_report.temperature_min.to_owned(),
        wind: new_report.wind.to_owned(),
        weather: new_report.weather.to_owned(),
    };
    let weather_detail = db.create_weather_report(data);
    match weather_detail {
        Ok(report) => Ok(Json(report)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/weather/<id>")]
pub fn get_weather_report(db: &State<MongoRepo>, id: String) -> Result<Json<Weather>, Status> {
    let id_db = id;
    if id_db.is_empty() {
        return Err(Status::BadRequest);
    };
    let weather_detail = db.get_weather_report(&id_db);
    match weather_detail {
        Ok(weather) => Ok(Json(weather)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/weather/<city>/<date>")]
pub fn get_weather_in_city(db: &State<MongoRepo>, city: String, mut date: String) -> Result<Json<Weather>, Status> {
    if date.is_empty() {
       date = chrono::Utc::now().date_naive().format("%Y-%m-%d").to_string();
    }
    if city.is_empty(){
        return Err(Status::BadRequest);
    };
    let weather_detail = db.get_weather_in_city(&city, &date);
    match weather_detail {
        Ok(report) => Ok(Json(report)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flights/departures/<city>/<date>")]
pub fn get_critical_departures(db: &State<MongoRepo>, city: String, mut date: String) -> Result<Json<Vec<Flight>>, Status> {
    if date.is_empty() {
        date = chrono::Utc::now().date_naive().format("%Y-%m-%d").to_string();
     }
    if city.is_empty() {
        return Err(Status::BadRequest);
    };
    let flight_details = db.get_critical_departures(&city, &date);
    match flight_details {
        Ok(flights) => Ok(Json(flights)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/flights/arrivals/<city>/<date>")]
pub fn get_critical_arrivals(db: &State<MongoRepo>, city: String, mut date: String) -> Result<Json<Vec<Flight>>, Status> {
    if date.is_empty() {
        date = chrono::Utc::now().date_naive().format("%Y-%m-%d").to_string();
     }
    if city.is_empty() {
        return Err(Status::BadRequest);
    };
    let flight_details = db.get_critical_arrivals(&city, &date);
    match flight_details {
        Ok(flights) => Ok(Json(flights)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/weather")]
pub fn get_all_weather_reports(db: &State<MongoRepo>) -> Result<Json<Vec<Weather>>, Status> {
    let weather = db.get_all_weather_reports();
    match weather {
        Ok(weather) => Ok(Json(weather)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/weather/<city>/<date>", data = "<new_report>")]
pub fn update_weather_report(db: &State<MongoRepo>, city: String, mut date: String, new_report: Json<Weather>,) -> Result<Json<Weather>, Status> {
    if date.is_empty() {
        date = chrono::Utc::now().date_naive().format("%Y-%m-%d").to_string();
     }
    if city.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Weather {
        id: None,
        date: date.to_owned(),
        city_code: new_report.city_code.to_owned(),
        precipitation: new_report.precipitation.to_owned(),
        temperature_max: new_report.temperature_max.to_owned(),
        temperature_min: new_report.temperature_min.to_owned(),
        wind: new_report.wind.to_owned(),
        weather: new_report.weather.to_owned(),
    };
    let update_result = db.update_weather_report(&city, &date, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_weather_info = db.get_weather_in_city(&city, &date);
                return match updated_weather_info {
                    Ok(weather) => Ok(Json(weather)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/weather/<id>")]
pub fn delete_weather_report(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    let id_db = id;
    if id_db.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_weather_report(&id_db);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Weather report successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}