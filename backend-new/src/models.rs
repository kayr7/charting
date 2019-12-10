extern crate serde_json;
use serde::{Serialize, Deserialize};
use super::schema::measurements;
use serde_aux::prelude::*;



#[database("mydb")]
pub struct MyDatabase(diesel::SqliteConnection);


#[derive(Serialize, Clone, Debug, Deserialize, Queryable, Insertable)]
#[table_name="measurements"]
pub struct Measurement {
    pub date: String,
    pub temperature: f32,
    pub schleimstruktur: String,
    pub geschlechtsverkehr: i32,
    pub mittelschmerz: i32,
    pub zwischenblutung: i32,
    pub blutung: String,
}


#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct MeasurementHelperRaw {
    pub date: String,
    pub temperature: f32,
    pub schleimstruktur: String,
    pub geschlechtsverkehr: bool,
    pub mittelschmerz: bool,
    pub zwischenblutung: bool,
    pub blutung: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct BoolHelperRaw {
    pub date: String,
    pub value: bool,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct StringHelperRaw {
    pub date: String,
    pub value: String,
}


#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct MeasurementHelper {
    #[serde(deserialize_with = "deserialize_struct_case_insensitive")]
    pub original: MeasurementHelperRaw,
}
