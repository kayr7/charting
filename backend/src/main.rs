#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate mut_static;
#[macro_use] extern crate lazy_static;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;
use serde::{Serialize, Deserialize};
extern crate serde_json;


extern crate rocket_cors;
use std::fs::OpenOptions;
use std::io::prelude::*;



use mut_static::MutStatic;

use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, // 2.
    Cors, CorsOptions // 3.
};
use rocket::http::Method;


fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_regex(&[ // 4.
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8000",
        "http://192.168.8.4:3000.*",
    ]);

    CorsOptions { // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", 
            "Content-Type",// 6.
        ]),
        allow_credentials: true, 
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}



#[derive(Serialize, Clone, Debug, Deserialize)]
struct Measurement {
    Date: String,
    Temperature: f64,
    Schleimstruktur: String,
    Geschlechtsverkehr: bool,
    Mittelschmerz: bool,
    Zwischenblutung: bool,
    Blutung: String,
}

lazy_static! {
    static ref MY_STATE: MutStatic<Vec<Measurement>> = MutStatic::new();
}



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/measurements")]
fn all_measurements() -> Json<Vec<Measurement>> {
    let ret_value = MY_STATE.read().unwrap();
    return Json(ret_value.to_vec())
}

#[post("/measurement", format = "text/plain", data = "<measurement>")]
fn new_measurement(measurement: Json<Measurement>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("my-file")
        .unwrap();

    let t = measurement.clone();

//    println!("{}", json!(t).to_string());

    if let Err(e) = writeln!(file, "{}", json!(t).to_string()) {
        eprintln!("Couldn't write to file: {}", e);
    }
    {
        let mut mut_handle = MY_STATE.write().unwrap();
        mut_handle.push(measurement.into_inner());
    }
}



fn main() {
    MY_STATE.set(vec![]).unwrap();

    rocket::ignite().mount("/", routes![index,
                                        all_measurements,
                                        new_measurement,
                                        ]).attach(make_cors()).launch();

}