#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_contrib::databases::diesel;

mod subject;

#[database("mysql_db")]
struct MysqlDB(diesel::MysqlConnection);

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
                subject::get_subjects,
                subject::get_subject,
                subject::post_subject,
                subject::put_subject,
                subject::delete_subject,
                subject::content::get_content,
                subject::content::post_content,
                subject::content::put_content,
                subject::content::delete_content
            ],
        )
        .launch();
}
