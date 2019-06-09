#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
extern crate quizapp;
extern crate diesel;

use rocket_contrib::json::{Json};
use rocket_contrib::serve::StaticFiles;
use quizapp::models::{Form, NewForm};
use quizapp::*;

#[get("/")]
fn get_forms() -> Json<Vec<Form>> {
    let connection = establish_connection();
    let forms = list_forms(&connection);
    Json(forms)
}

#[post("/", format = "json", data = "<new_form>")]
fn post_form(new_form: Json<NewForm>) -> Json<Form> {
    let connection = establish_connection();
    let form = create_form(&connection, new_form.title, new_form.description);
    Json(form)
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("web"))
        .mount("/forms", routes![get_forms, post_form])
        .launch();
}
