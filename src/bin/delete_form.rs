extern crate quizapp;
extern crate diesel;

use self::diesel::prelude::*;
use self::quizapp::*;
use std::env::args;

fn main() {
    use quizapp::schema::forms::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(forms.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting forms");

    println!("Deleted {} forms", num_deleted);
}