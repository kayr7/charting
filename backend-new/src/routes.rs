extern crate chrono;



use crate::models::Measurement;
use crate::models::MeasurementHelper;
use crate::models::MeasurementHelperRaw;
use crate::models::BoolHelperRaw;
use crate::models::MyDatabase;
use crate::schema::measurements::columns::geschlechtsverkehr;
use crate::schema::measurements::columns::mittelschmerz;
use crate::schema::measurements::columns::zwischenblutung;
use crate::schema::measurements::columns::date;
use crate::schema::measurements;


use chrono::{NaiveDate};

use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;
use crate::diesel::ExpressionMethods;

use rocket_contrib::json::Json;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;




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



#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/measurements")]
pub fn all_measurements(conn: MyDatabase) -> Json<Vec<MeasurementHelperRaw>> {
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
pub fn new_measurement(conn: MyDatabase, measurement: Json<MeasurementHelperRaw>) {

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


fn update_g(v: i32, dat: String, conn: MyDatabase) {
    let result = diesel::update(measurements::table.filter(date.eq(dat)))
                        .set(geschlechtsverkehr.eq(v))
                        .execute(&*conn);
    match result {
        Ok(_) => {},
        Err(e) => println!("error inserting into Database {}", e)
    };
}

#[post("/update_gv", format = "text/plain", data = "<measurement>")]
pub fn update_gv(conn: MyDatabase, measurement: Json<BoolHelperRaw>) {
    println!("{:?}", measurement);
    let new_date = date_from_app_to_sqlite(&measurement.date);
    println!("new_date: {:?}", new_date);
    update_g(if measurement.value { 1 } else { 0 }, new_date, conn);
}


fn update_ms(v: i32, dat: String, conn: MyDatabase) {
    let result = diesel::update(measurements::table.filter(date.eq(dat)))
                        .set(mittelschmerz.eq(v))
                        .execute(&*conn);
    match result {
        Ok(_) => {},
        Err(e) => println!("error inserting into Database {}", e)
    };
}
#[post("/update_ms", format = "text/plain", data = "<measurement>")]
pub fn update_mittelschmerz(conn: MyDatabase, measurement: Json<BoolHelperRaw>) {
    println!("{:?}", measurement);
    let new_date = date_from_app_to_sqlite(&measurement.date);
    println!("new_date: {:?}", new_date);
    update_ms(if measurement.value { 1 } else { 0 }, new_date, conn);
}


fn update_z(v: i32, dat: String, conn: MyDatabase) {
    let result = diesel::update(measurements::table.filter(date.eq(dat)))
                        .set(zwischenblutung.eq(v))
                        .execute(&*conn);
    match result {
        Ok(_) => {},
        Err(e) => println!("error inserting into Database {}", e)
    };
}
#[post("/update_zb", format = "text/plain", data = "<measurement>")]
pub fn update_zwischenblutung(conn: MyDatabase, measurement: Json<BoolHelperRaw>) {
    println!("{:?}", measurement);
    let new_date = date_from_app_to_sqlite(&measurement.date);
    println!("new_date: {:?}", new_date);
    update_z(if measurement.value { 1 } else { 0 }, new_date, conn);
}


#[get("/update_from_file")]
pub fn file_update(conn: MyDatabase) -> &'static str {
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

