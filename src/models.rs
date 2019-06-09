use serde_derive::{Deserialize, Serialize};

use super::schema::forms;

#[derive(Queryable, Serialize)]
pub struct Form {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name="forms"]
pub struct NewForm<'a> {
    pub title: &'a str,
    pub description: &'a str,
}