extern crate diesel;
use diesel::{Connection, PgConnection};

pub fn db_connection()->PgConnection{
    let connect_urls = std::env::var("DATABASE_URL")
        .expect("URL RETRIEVAL FAILED.");
    return PgConnection::establish(&connect_urls)
        .expect("DATABASE CONNECTION FAILED.");
}