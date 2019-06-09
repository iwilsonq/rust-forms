extern crate quizapp;
extern crate diesel;

use self::quizapp::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use quizapp::schema::forms::dsl::*;

    let connection = establish_connection();
    let results = forms.filter(published.eq(true))
        .limit(5)
        .load::<Form>(&connection)
        .expect("Error loading forms");

    println!("Displaying {} forms", results.len());
    for form in results {
        println!("{}", form.title);
        println!("----------\n");
        println!("{}", form.description);
    }
}