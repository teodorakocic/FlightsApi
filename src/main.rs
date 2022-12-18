mod api;
mod model;
mod repository;

#[macro_use]
extern crate rocket;

use api::{flight_api::{create_flight, get_flight, delete_flight, get_flights_by_destination, get_flights_by_origin_date, get_flights_in_price_range, get_flights_with_connection_for_destination, get_flights_with_passengers_numbers, delete_all_flights, get_all_flights, update_flight_date_travel_class}, 
    weather_api::{create_weather_report, get_weather_report, get_weather_in_city, get_critical_departures, delete_weather_report, get_all_weather_reports, get_critical_arrivals, update_weather_report}};
use repository::database::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_flight])
        .mount("/", routes![get_flight])
        .mount("/", routes![get_flights_by_destination])
        .mount("/", routes![get_flights_by_origin_date])
        .mount("/", routes![get_flights_in_price_range])
        .mount("/", routes![get_flights_with_connection_for_destination])
        .mount("/", routes![get_flights_with_passengers_numbers])
        .mount("/", routes![get_all_flights])
        .mount("/", routes![update_flight_date_travel_class])
        .mount("/", routes![delete_flight])
        .mount("/", routes![delete_all_flights])
        .mount("/", routes![create_weather_report])
        .mount("/", routes![get_weather_report])
        .mount("/", routes![get_weather_in_city])
        .mount("/", routes![get_critical_departures])
        .mount("/", routes![get_critical_arrivals])
        .mount("/", routes![get_all_weather_reports])
        .mount("/", routes![update_weather_report])
        .mount("/", routes![delete_weather_report])
}
