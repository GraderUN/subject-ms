/*
subject.rs

The Subject microservice relates all information of a particular subject within Grader.

Each subject has select details.
A subject may have a detailed content.
A subject may be taught by several teachers.
*/
use super::models::*;
use super::schema::*;

use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

#[get("/subject")]
pub fn get_subjects(db: super::MysqlDB) -> Json<Vec<Subject>> {
    //! "The get subjects path"
    Json(
        subject::table
            .load::<Subject>(&*db)
            .expect("GET Subjects ERROR"),
    )
    // if let Ok(r) = subject::table.load(&*db) {
    //     return r;
    // } else {
    //     return Json(json!());
    // }
}

#[get("/subject/<id>")]
pub fn get_subject(db: super::MysqlDB, id: u32) -> JsonValue {
    let response = subject::table
        .filter(subject::id.eq(id))
        .load::<Subject>(&*db);

    if let Ok(rows) = response {
        json!(rows[0])
    } else {
        json!({"response" : "Error completing request",
            "path":"GET /subject/id"})
    }
}

#[post("/subject", data = "<subject>")]
pub fn post_subject(db: super::MysqlDB, subject: Json<PostSubject>) -> JsonValue {
    let response = diesel::insert_into(subject::table)
        .values(&*subject)
        .execute(&*db);

    if let Ok(rows) = response {
        json!({"rows": rows,
            "response" : "Success",
            "path":"POST /subject/id"})
    } else {
        json!({"response" : "Error completing request",
            "path":"POST /subject/id"})
    }
}

#[put("/subject/<id>")]
pub fn put_subject(db: super::MysqlDB, id: u32) -> () {
    //! "PUT a subject"
}

#[delete("/subject/<id>")]
pub fn delete_subject(db: super::MysqlDB, id: u32) -> () {
    //! "DELETE a subject"
}

pub mod content {

    #[get("/subject/content")]
    pub fn get_content() -> &'static str {
        "The get content path"
    }

    #[post("/subject/content")]
    pub fn post_content() -> &'static str {
        "The post content path"
    }

    #[put("/subject/content")]
    pub fn put_content() -> &'static str {
        "The put content path"
    }

    #[delete("/subject/content")]
    pub fn delete_content() -> &'static str {
        "The delete content path"
    }
}

pub mod teacher {}
