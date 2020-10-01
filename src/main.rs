#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
//use rocket_contrib::databases::diesel;
pub mod models;
pub mod schema;

mod routes;

#[database("mysql_db")]
pub struct MysqlDB(diesel::MysqlConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .attach(MysqlDB::fairing())
        .mount(
            "/",
            routes![
                index,
                routes::get_subjects,
                routes::get_subject,
                routes::post_subject,
                routes::put_subject,
                routes::delete_subject,
                routes::content::get_content,
                routes::content::post_content,
                routes::content::put_content,
                routes::content::delete_content
            ],
        )
        .launch();
}
