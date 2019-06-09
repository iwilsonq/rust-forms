extern crate quizapp;
extern crate diesel;

use self::diesel::prelude::*;
use self::quizapp::*;
use self::models::Form;
use std::env::args;

fn main() {
    use quizapp::schema::forms::dsl::{forms, published};

    let id = args().nth(1).expect("publish_form requires a form id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let form = diesel::update(forms.find(id))
        .set(published.eq(true))
        .get_result::<Form>(&connection)
        .expect(&format!("Unable to find form {}", id));
    println!("Published form {}", form.title);
}