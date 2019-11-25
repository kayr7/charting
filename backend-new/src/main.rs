#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;

use rocket_contrib::json::{Json};
use rocket_contrib::json;
//use rocket_contrib::databases::diesel;

extern crate serde_json;
use serde::{Serialize, Deserialize};
use serde_json::{Result};

extern crate rocket_cors;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, // 2.
    Cors, CorsOptions // 3.
};

use std::fs::OpenOptions;
use std::fs::File;

use std::io::prelude::*;
use std::io::{BufReader, Read, BufRead};

use mut_static::MutStatic;

use rocket::http::Method;

use rocket::State;


extern crate chrono;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;
use std::{cell::RefCell, sync::Mutex};

use crate::diesel::RunQueryDsl;
use diesel::sql_query;
use diesel::prelude::*;



table! {
    use diesel::sql_types::*;
    measurements (date) {
        date -> Text,
        Temperature -> Double,
        Schleimstruktur -> Text,
        Geschlechtsverkehr -> Integer,
        Mittelschmerz -> Integer,
        Zwischenblutung -> Integer,
        Blutung -> Text,
    }
}

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



#[derive(Serialize, Clone, Debug, Deserialize, Queryable)]
struct Measurement {
    Date: String,
    Temperature: f64,
    Schleimstruktur: String,
    Geschlechtsverkehr: i32,
    Mittelschmerz: i32,
    Zwischenblutung: i32,
    Blutung: String,
}


#[database("mydb")]
struct MyDatabase(diesel::SqliteConnection);
impl MyDatabase {
    pub fn initialize_db(&self) {
//        self.execute_batch(SCHEMA).expect("create tables");
    }

/*    fn task_from_row(row: &Row) -> Task {
        Task::new(
            row.get("id"),
            row.get("name"),
            vec![],
            row.get("due"),
            row.get::<&str, f64>("hours") as f32,
        )
    }

    fn tags_for_task(&self, id: u32) -> Vec<String> {
        let mut stmt = self.0
            .prepare("SELECT tag from tasks_tags WHERE task = ?;")
            .expect("query tags statement");
        let tags: Result<Vec<String>, rusqlite::Error> =
            stmt.query_map(&[&id], |row| row.get(0)).unwrap().collect();
        tags.expect("query tags results")
    }

    fn delete_task_internal(tx: &rusqlite::Transaction, id: u32) {
        tx.execute("DELETE FROM tasks_tags WHERE task = ?;", &[&id])
            .expect("delete task tags");
        tx.execute("DELETE FROM tasks WHERE id = ?;", &[&id])
            .expect("delete task");
    }*/
}



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/measurements")]
fn all_measurements(conn: MyDatabase) -> Json<Vec<Measurement>> {
/*    let mut stmt = conn.prepare("SELECT date, 
                                        Temperature,
                                        Schleimstruktur,
                                        Geschlechtsverkehr,
                                        Mittelschmerz,
                                        Zwischenblutung,
                                        Blutung FROM measurements");
    let measurement_iter = stmt.query_map(params![], |row| {
        Measurement {
            Date:  date_from_sqlite_to_app(row.get(0)?),
            Temperature: row.get(1)?,
            Schleimstruktur: row.get(2)?,
            Geschlechtsverkehr: row.get(3)?,
            Mittelschmerz: row.get(4)?,
            Zwischenblutung: row.get(5)?,
            Blutung: row.get(6)?,
        }
    })?;*/
    let results = measurements::table
        .load::<Measurement>(&*conn)
        .expect("Error loading posts");

//    for r in results {
//        println!("{:?}", r);
//    }

/*    let ret_value = vec!(Measurement{ 
        Date: "2019-11-25".to_string(),
        Temperature: 37.3,
        Schleimstruktur: "".to_string(),
        Geschlechtsverkehr: 1,
        Mittelschmerz: 1,
        Zwischenblutung: 1,
        Blutung: "".to_string(),
    });*/
    return Json(results)
}

#[post("/measurement", format = "text/plain", data = "<measurement>")]
fn new_measurement(measurement: Json<Measurement>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("my-file")
        .unwrap();

    let t = measurement.clone();

    if let Err(e) = writeln!(file, "{}", json!(t).to_string()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn read_from_file(filename: String) -> Vec<Measurement>{
    let mut measurement_vector: Vec<Measurement> = vec![];
    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            println!("{:?}", line);
            if let Ok(m) = serde_json::from_str(&line.unwrap()) {
                measurement_vector.push(m);
            }

        }
    }
    return measurement_vector;
}


/*
fn create_tables(conn: &Connection) {
    println!("{:?}", conn.execute(
        "CREATE TABLE if not exists measurements (
             date TEXT PRIMARY KEY,
             Temperature REAL not null,
             Schleimstruktur TEXT,
             Geschlechtsverkehr INTEGER,
             Mittelschmerz INTEGER,
             Zwischenblutung INTEGER,
             Blutung TEXT
         )",
        NO_PARAMS,
    ));
}*/

fn date_from_sqlite_to_app(in_date: &str) -> String {
    let date = NaiveDate::parse_from_str(in_date, "%Y-%m-%d").unwrap();
    let ret_val = date.format("%d.%m.%Y").to_string().clone();
    return ret_val
}

fn insert_measurements(measurements: Vec<Measurement>) {
    for m in measurements {
        let date = NaiveDate::parse_from_str(&m.Date, "%d.%m.%Y").unwrap();
        let date_str = date.format("%Y-%m-%d").to_string();
/*        let result = conn.execute(
            "INSERT INTO measurements (
                                        date,
                                        Temperature,
                                        Schleimstruktur,
                                        Geschlechtsverkehr,
                                        Mittelschmerz,
                                        Zwischenblutung,
                                        Blutung) 
                        values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &date_str,
                m.Temperature,
                &m.Schleimstruktur,
                if m.Geschlechtsverkehr { 1 } else { 0 },
                if m.Mittelschmerz { 1 } else { 0 },
                if m.Zwischenblutung {1 } else { 0 },
                &m.Blutung
            ],
        );*/
//        let last_id : String = conn.last_insert_rowid().to_string();
//        println!("{:?}", last_id);
    }
}


fn main() {
//    let conn = Connection::open("charting.db").unwrap();
    let fairing = MyDatabase::fairing();

//    create_tables(&conn);

//    let past_measurements = read_from_file("./my-file".to_string());
//    insert_measurements(past_measurements, &conn);


    rocket::ignite().mount("/", routes![index,
                                        all_measurements,
                                        new_measurement,
                                        ])
                                        .attach(make_cors())
                                        .attach(MyDatabase::fairing())
/*                                        .attach("Initialize Database Schema", |rocket| {
                                                let conn = MyDatabase::get_one(&rocket).expect("database connection");
                                                conn.initialize_db();
                                                Ok(rocket)})*/
                                        .launch();

}