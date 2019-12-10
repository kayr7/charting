#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;
pub mod routes;


use rocket::http::Method;

extern crate serde_json; 

extern crate rocket_cors;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, // 2.
    Cors, CorsOptions // 3.
};

use self::models::MyDatabase;



use crate::routes::static_rocket_route_info_for_update_zwischenblutung;
use crate::routes::static_rocket_route_info_for_update_blutung;
use crate::routes::static_rocket_route_info_for_update_mittelschmerz;
use crate::routes::static_rocket_route_info_for_update_schleim;
use crate::routes::static_rocket_route_info_for_update_gv;
use crate::routes::static_rocket_route_info_for_file_update;
use crate::routes::static_rocket_route_info_for_new_measurement;
use crate::routes::static_rocket_route_info_for_all_measurements;
use crate::routes::static_rocket_route_info_for_index;


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







fn main() {
    rocket::ignite().mount("/", routes![index,
                                        all_measurements,
                                        new_measurement,
                                        file_update,
                                        update_gv,
                                        update_mittelschmerz,
                                        update_zwischenblutung,
                                        update_blutung,
                                        update_schleim,
                                        ])
                                        .attach(make_cors())
                                        .attach(MyDatabase::fairing())
                                        .launch();

}