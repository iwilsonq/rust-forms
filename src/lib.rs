#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Form, NewForm};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_form<'a>(conn: &PgConnection, title: &'a str, description: &'a str) -> Form {
    use schema::forms;

    let new_form = NewForm {
        title,
        description,
    };

    diesel::insert_into(forms::table)
        .values(&new_form)
        .get_result(conn)
        .expect("Error saving new form")
}

pub fn list_forms<'a>(conn: &PgConnection) -> Vec<Form> {
    use schema::forms::dsl::*;
    forms.limit(10)
        .load::<Form>(conn)
        .expect("Error loading forms")
}