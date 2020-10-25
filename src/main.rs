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
                routes::subject::get_subjects,
                routes::subject::get_subject,
                routes::subject::get_grade,
                routes::subject::post_subject,
                routes::subject::put_subject,
                routes::subject::delete_subject,
                routes::content::get_content,
                routes::content::put_content,
                routes::teacher::post_teacher,
                routes::teacher::delete_teacher,
                routes::teacher::get_teacher_subjects,
                routes::teacher::get_subject_teachers,
            ],
        )
        .launch();
}
