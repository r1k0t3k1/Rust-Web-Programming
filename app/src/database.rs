use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::config::Config;

pub fn establish_connection() -> PgConnection {
    //dotenv().ok();
    //let database_url = env::var("DATABASE_URL")
    //    .expect("DATABASE_URL must be set.");
    let config = Config::new();
    let database_url = config.map["DB_URL"].as_str().unwrap();

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

}
