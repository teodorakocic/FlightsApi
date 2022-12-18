extern crate dotenv;
use std::cmp::Ordering;

use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, DeleteResult, UpdateResult},
    sync::{Client, Collection},
};
use crate::model::flight_model::Flight;
use crate::model::weather_model::Weather;

const MONGOURI: &str = "mongodb+srv://teodorakocic:Informatika99!@cluster0.k9j3nyx.mongodb.net/?retryWrites=true&w=majority";

pub struct MongoRepo {
    flights: Collection<Flight>,
    weather: Collection<Weather>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = String::from(MONGOURI);
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let flights: Collection<Flight> = db.collection("Flight");
        let weather: Collection<Weather> = db.collection("Weather");
        MongoRepo { flights, weather }
    }

    pub fn create_flight(&self, new_flight: Flight) -> Result<InsertOneResult, Error> {
        let new_document = Flight {
            id: None,
            origin: new_flight.origin,
            destination: new_flight.destination,
            departure_date: new_flight.departure_date,
            return_date: new_flight.return_date,
            adults: new_flight.adults,
            children: new_flight.children,
            infants: new_flight.infants,
            travel_class: new_flight.travel_class,
            non_stop: new_flight.non_stop,
            currency_code: new_flight.currency_code,
            max_price: new_flight.max_price,
            max_offers: new_flight.max_offers,
        };
        let flight = self
            .flights
            .insert_one(new_document, None)
            .ok()
            .expect("Error creating flight");
        Ok(flight)
    }

    pub fn get_flight(&self, id: &String) -> Result<Flight, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let flight_detail = self
            .flights
            .find_one(filter, None)
            .ok()
            .expect("Error getting flight's detail");
        Ok(flight_detail.unwrap())
    }

    pub fn get_flights_by_destination(&self, destination: &String) -> Result<Vec<Flight>, Error> {
        let filter = doc! {"destination": destination.to_uppercase().to_owned()};
        let cursor = self
            .flights
            .find(filter, None)
            .ok()
            .expect("Error getting flights with specified destination!");
            let flights =  cursor.map(|doc| doc.unwrap()).collect();
        Ok(flights)
    }

    pub fn get_flights_by_origin_date(&self, origin: &String, date: &String) -> Result<Vec<Flight>, Error> {
        let filter = doc! {"origin": origin.to_uppercase().to_owned(), "return_date": date.to_owned()};
        let cursor = self
            .flights
            .find(filter, None)
            .ok()
            .expect("Error getting flights with specified origin for wanted return date!");
            let flights =  cursor.map(|doc| doc.unwrap()).collect();
        Ok(flights)
    }

    pub fn get_flights_in_price_range(&self, _max_price: f32, _min_price: f32, currency: &String) -> Result<Vec<Flight>, Error> {
        let filter = doc! {"max_price": { "$lte": _max_price, "$gte": _min_price }, "currency_code": currency.to_uppercase().to_owned()};
        let cursor = self
            .flights
            .find(filter, None)
            .ok()
            .expect("Error getting flights within specified range!");
            let flights =  cursor.map(|doc| doc.unwrap()).collect();
        Ok(flights)
    }

    pub fn get_flights_with_connection_for_destination(&self, destination: &String, stop: bool) -> Result<Vec<Flight>, Error> {
        let filter = doc! {"destination": destination.to_uppercase().to_owned(), "non_stop": stop};
        let cursor = self
            .flights
            .find(filter, None)
            .ok()
            .expect("Error getting flights with connection for specified destination!");
            let flights =  cursor.map(|doc| doc.unwrap()).collect();
        Ok(flights)
    }

    pub fn get_flights_with_passengers_numbers(&self, adults: i32, children: i32, infants: i32) -> Result<Vec<Flight>, Error> {
        let filter = doc! {"adults": adults, "children": { "$lte": children }, "infants": { "$lte": infants }};
        let cursor = self
            .flights
            .find(filter, None)
            .ok()
            .expect("Error getting flights within specified number of passengers!");
            let flights =  cursor.map(|doc| doc.unwrap()).collect();
        Ok(flights)
    }

    pub fn get_all_flights(&self) -> Result<Vec<Flight>, Error> {
        let cursors = self
            .flights
            .find(None, None)
            .ok()
            .expect("Error getting list of flights");
        let flights = cursors.map(|doc| doc.unwrap()).collect();
        Ok(flights)
    }

    pub fn update_flight_date_travel_class(&self, id: &String, new_flight: Flight) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_flight.id,
                    "travel_class": new_flight.travel_class,
                    "return_date": new_flight.return_date
                },
        };
        let updated_doc = self
            .flights
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating flight");
        Ok(updated_doc)
    }

    pub fn delete_flight(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let flight_detail = self
            .flights
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting flight");
        Ok(flight_detail)
    }
    
    pub fn delete_all_flights(&self) -> Result<DeleteResult, Error> {
        let filter = doc! {};
        let flight_detail = self
            .flights
            .delete_many(filter, None)
            .ok()
            .expect("Error deleting flight");
        Ok(flight_detail)
    }


    pub fn create_weather_report(&self, new_report: Weather) -> Result<InsertOneResult, Error> {
        let new_document = Weather {
            id: None,
            date: new_report.date,
            city_code: new_report.city_code,
            precipitation: new_report.precipitation,
            temperature_max: new_report.temperature_max,
            temperature_min: new_report.temperature_min,
            wind: new_report.wind,
            weather: new_report.weather,
        };
        let report = self
            .weather
            .insert_one(new_document, None)
            .ok()
            .expect("Error creating wetaher report");
        Ok(report)
    }

    pub fn get_weather_report(&self, id: &String) -> Result<Weather, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let weather_detail = self
            .weather
            .find_one(filter, None)
            .ok()
            .expect("Error getting weather report!");
        Ok(weather_detail.unwrap())
    }

    pub fn get_weather_in_city(&self, city: &String, date: &String) -> Result<Weather, Error> {
        let filter = doc! {"date": date.to_owned(), "city_code": city.to_uppercase().to_owned()};
        let weather_detail = self
            .weather
            .find_one(filter, None)
            .ok()
            .expect("Error getting details on waether in specified city!");
        Ok(weather_detail.unwrap())
    }

    pub fn get_critical_departures(&self, city: &String, date: &String) -> Result<Vec<Flight>, Error> {
        let filter = doc! {"date": date.to_owned(), "city_code": city.to_uppercase().to_owned(), "precipitation": {"$gte": 7.4}, "wind": {"$gt": 1.2}};
        let cursor = self
            .weather
            .find(filter, None)
            .ok()
            .expect("Error getting critical departures!");
        let weather_reports: Vec<Weather> =  cursor.map(|doc| doc.unwrap()).collect();

        let filter2 = doc! {"origin": city.to_uppercase().to_owned(), "departure_date": date.to_owned()};
        let cursor2 = self
            .flights
            .find(filter2, None)
            .ok()
            .expect("Error getting critical departures!");
        let mut flights: Vec<Flight> = cursor2.map(|doc| doc.unwrap()).collect();
        let mut indicator = false;

        for ind in 0..flights.len() {
            for w in 0..weather_reports.len() {
                if flights[ind].origin.to_owned().cmp(&weather_reports[w].city_code.to_owned()) == Ordering::Equal {
                    indicator = true;
                }
            }
            if !indicator {
                flights.remove(ind);
            }
            indicator = false;
        } 
        Ok(flights)
    }

    pub fn get_critical_arrivals(&self, city: &String, date: &String) -> Result<Vec<Flight>, Error> {
        let filter = doc! {"date": date.to_owned(), "city_code": city.to_uppercase().to_owned(), "weather": {"$in": ["snow", "rain"]}};
        let cursor = self
            .weather
            .find(filter, None)
            .ok()
            .expect("Error getting critical arrivals!");
        let weather_reports: Vec<Weather> =  cursor.map(|doc| doc.unwrap()).collect();

        let filter2 = doc! {"destination": city.to_uppercase().to_owned(), "departure_date": date.to_owned()};
        let cursor2 = self
            .flights
            .find(filter2, None)
            .ok()
            .expect("Error getting critical arrivals!");
        let mut flights: Vec<Flight> = cursor2.map(|doc| doc.unwrap()).collect();
        let mut indicator = false;

        for ind in 0..flights.len() {
            for w in 0..weather_reports.len() {
                if flights[ind].destination.to_owned().cmp(&weather_reports[w].city_code.to_owned()) == Ordering::Equal {
                    indicator = true;
                }
            }
            if !indicator {
                flights.remove(ind);
            }
            indicator = false;
        } 
        Ok(flights)
    }

    pub fn get_all_weather_reports(&self) -> Result<Vec<Weather>, Error> {
        let cursors = self
            .weather
            .find(None, None)
            .ok()
            .expect("Error getting list of flights");
        let weather = cursors.map(|doc| doc.unwrap()).collect();
        Ok(weather)
    }

    pub fn update_weather_report(&self, city: &String, date: &String, new_report: Weather) -> Result<UpdateResult, Error> {
        let filter = doc! {"city_code": city.to_uppercase().to_owned(), "date": date.to_owned()};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_report.id,
                    "precipitation": new_report.precipitation,
                    "temperature_max": new_report.temperature_max,
                    "temperature_min": new_report.temperature_min,
                    "wind": new_report.wind,
                    "weather": new_report.weather
                },
        };
        let updated_doc = self
            .weather
            .update_many(filter, new_doc, None)
            .ok()
            .expect("Error updating weather reports!");
        Ok(updated_doc)
    }

    pub fn delete_weather_report(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let weather_detail = self
            .weather
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting weather report");
        Ok(weather_detail)
    }
}
