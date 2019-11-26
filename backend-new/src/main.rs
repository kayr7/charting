#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket_contrib::json::{Json};
//use rocket_contrib::databases::diesel;

extern crate serde_json;

extern crate rocket_cors;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, // 2.
    Cors, CorsOptions // 3.
};

use std::fs::File;

use std::io::{BufReader, BufRead};

use rocket::http::Method;

extern crate chrono;
use chrono::{NaiveDate};

use crate::diesel::RunQueryDsl;
use crate::diesel::query_dsl::methods::OrderDsl;
use crate::schema::measurements::columns::date;

pub mod schema;
pub mod models;

use self::models::Measurement;
use self::models::MeasurementHelper;
use self::models::MeasurementHelperRaw;

use std::fs::OpenOptions;
use std::io::Write;

use self::schema::*;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_regex(&[ // 4.
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8001",
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






#[database("mydb")]
struct MyDatabase(diesel::SqliteConnection);



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/measurements")]
fn all_measurements(conn: MyDatabase) -> Json<Vec<MeasurementHelperRaw>> {
    let results = measurements::table
        .order(date)
        .load::<Measurement>(&*conn)
        .expect("Error loading posts");

    let mut ret_vector: Vec<MeasurementHelperRaw> = vec![];
    for r in results {
        let new_date = date_from_sqlite_to_app(&r.date);
        let m = MeasurementHelperRaw {
            date: new_date,
            temperature: r.temperature,
            schleimstruktur: r.schleimstruktur,
            geschlechtsverkehr: r.geschlechtsverkehr != 0,
            mittelschmerz: r.mittelschmerz != 0,
            zwischenblutung: r.zwischenblutung != 0,
            blutung: r.blutung,
        };
        ret_vector.push(m);
    }

    return Json(ret_vector);
}


fn insert_measurement(m: Measurement, conn: MyDatabase) {
    if m.temperature > 34. && m.temperature < 42. {
        let result = diesel::insert_into(measurements::table)
                            .values(&m).execute(&*conn);
        match result {
            Ok(_) => {},
            Err(e) => println!("error inserting into Database {}: {:?}", e, m)
        };

    }
}

#[post("/measurement", format = "text/plain", data = "<measurement>")]
fn new_measurement(conn: MyDatabase, measurement: Json<MeasurementHelperRaw>) {

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("my-file")
        .unwrap();

    let t = measurement.clone();

    if let Err(e) = writeln!(file, "{}", json!(t).to_string()) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("{:?}", measurement);
    let new_date = date_from_app_to_sqlite(&measurement.date);
    println!("new_date: {:?}", new_date);
    let new_measurement = Measurement {
        date : new_date,
        temperature: measurement.temperature,
        schleimstruktur: measurement.schleimstruktur.clone(),
        geschlechtsverkehr: if measurement.geschlechtsverkehr { 1 } else { 0 },
        mittelschmerz: if measurement.mittelschmerz { 1 } else { 0 },
        zwischenblutung: if measurement.zwischenblutung { 1 } else { 0 },
        blutung: measurement.blutung.clone(),

    };
    insert_measurement(new_measurement, conn);
}

#[get("/update_from_file")]
fn file_update(conn: MyDatabase) -> &'static str {
    if let Ok(file) = File::open("my-file") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let helper_string = format!("{{ \"original\" : {} }}", &line.unwrap());
            let res: Result<MeasurementHelper, _> = serde_json::from_str(&helper_string);
            match res {
                Ok(m) => {
                    let new_date = date_from_app_to_sqlite(&m.original.date);
                    let new_measurement = Measurement {
                        date : new_date,
                        temperature: m.original.temperature,
                        schleimstruktur: m.original.schleimstruktur.clone(),
                        geschlechtsverkehr: if m.original.geschlechtsverkehr { 1 } else { 0 },
                        mittelschmerz: if m.original.mittelschmerz { 1 } else { 0 },
                        zwischenblutung: if m.original.zwischenblutung { 1 } else { 0 },
                        blutung: m.original.blutung.clone(),
                    };
                    if new_measurement.temperature > 34. && new_measurement.temperature < 42. {
                        let result = diesel::insert_into(measurements::table)
                            .values(&new_measurement)
                            .execute(&*conn);

                        match result {
                            Ok(_) => {},
                            Err(e) => println!("error inserting into Database {}: {:?}", e, new_measurement)
                        };

                    }
                }
                Err(e) => println!("Error Converting Json: {:?}", e)
            }
        }
    }
    return "done"
}



fn date_from_sqlite_to_app(in_date: &str) -> String {
    let n_date = NaiveDate::parse_from_str(in_date, "%Y-%m-%d").unwrap();
    let ret_val = n_date.format("%d.%m.%Y").to_string().clone();
    return ret_val
}

fn date_from_app_to_sqlite(in_date: &str) -> String {
    let n_date = NaiveDate::parse_from_str(in_date, "%d.%m.%Y").unwrap();
    let ret_val = n_date.format("%Y-%m-%d").to_string().clone();
    return ret_val
}



fn main() {
    rocket::ignite().mount("/", routes![index,
                                        all_measurements,
                                        new_measurement,
                                        file_update,
                                        ])
                                        .attach(make_cors())
                                        .attach(MyDatabase::fairing())
                                        .launch();

}