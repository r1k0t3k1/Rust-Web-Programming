use diesel::prelude::*;
use crate::config::Config;
use actix_web::dev::Payload;
use actix_web::error::ErrorServiceUnavailable;
use actix_web::{ Error, FromRequest, HttpRequest };
use futures::future::{ Ready, ok, err };
use once_cell::sync::Lazy;
use diesel::{
    r2d2::{ Pool, ConnectionManager, PooledConnection },
    pg::PgConnection,
};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_connection: PgPool,
}

pub static DB_CONNECTION : Lazy<DbConnection> = Lazy::new(|| {
    let config = Config::new();
    let connction_string = config.map["DB_URL"].as_str().unwrap();
    DbConnection {
        db_connection: PgPool::builder()
            .max_size(8)
            .build(ConnectionManager::new(connction_string))
            .expect("Failed to create db connection pool{}.")
    }
});

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    DB_CONNECTION.db_connection.get().unwrap()
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;

    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future {
        match DB_CONNECTION.db_connection.get() {
            Ok(connection) => return ok(DB { connection }),
            Err(_) => return err(ErrorServiceUnavailable("Could not establish connection to database.")),
        }  
    }
}
