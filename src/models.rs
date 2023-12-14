// This file contains everyting related to models and the database

use salvo::oapi::ToSchema;
use surrealdb::{sql::Thing, Surreal};
use serde::{Deserialize, Serialize};
use once_cell::sync::OnceCell;

pub static DB_POOL: OnceCell<Surreal<surrealdb::engine::local::Db>> = OnceCell::new();  // Object to store database connection
pub fn db_pool() -> &'static Surreal<surrealdb::engine::local::Db> {                    // Returns a reference to a database connection
    DB_POOL.get().unwrap()
}

/// Describes a cabin. If booked is not None/null, then the cabin is booked
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Cabin {
    pub name: String,
    pub booked: Option<String>,
}

/// User
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
    pub admin: bool,
}

/// A generic record in the Database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: Thing,
}

// Runs at startup to fill the in memory database with cabins and users
pub async fn populate_db_at_startup(db: &Surreal<surrealdb::engine::local::Db>) {

    // Create cabins
    let _created: Vec<Record> = db
        .create("cabin")
        .content(Cabin {
            name: "Cabin 1".into(),
            booked: Some("guest".into())
        })
        .await.unwrap();

    let _created: Vec<Record> = db
        .create("cabin")
        .content(Cabin {
            name: "Cabin 2".into(),
            booked: None
        })
        .await.unwrap();

    let _created: Vec<Record> = db
        .create("cabin")
        .content(Cabin {
            name: "Cabin 3".into(),
            booked: None
        })
        .await.unwrap();

    let _created: Vec<Record> = db
        .create("cabin")
        .content(Cabin {
            name: "Cabin 4".into(),
            booked: None
        })
        .await.unwrap();

    let _created: Vec<Record> = db
        .create("cabin")
        .content(Cabin {
            name: "Cabin 5".into(),
            booked: None
        })
        .await.unwrap();

    // create users
    let _created: Vec<Record> = db
        .create("user")
        .content(User {
            name: "admin".into(),
            password: "pwd".into(),
            admin: true
        })
        .await.unwrap();

    let _created: Vec<Record> = db
        .create("user")
        .content(User {
            name: "user1".into(),
            password: "pwd".into(),
            admin: false
        })
        .await.unwrap();

    let _created: Vec<Record> = db
        .create("user")
        .content(User {
            name: "user2".into(),
            password: "pwd".into(),
            admin: false
        })
        .await.unwrap();

    let _created: Vec<Record> = db
        .create("user")
        .content(User {
            name: "guest".into(),
            password: "pwd".into(),
            admin: false
        })
        .await.unwrap();
}
