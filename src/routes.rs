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

#[get("/subjects")]
pub fn get_subjects(db: super::MysqlDB) -> JsonValue {
    let response = subject::table.load::<Subject>(&*db);

    if let Ok(rows) = response {
        json!(rows)
    } else {
        json_error("GET /subjects")
    }
}

#[get("/subject/<id>")]
pub fn get_subject(db: super::MysqlDB, id: u32) -> JsonValue {
    let response = subject::table
        .filter(subject::id.eq(id))
        .load::<Subject>(&*db);

    if let Ok(rows) = response {
        json!(rows[0])
    } else {
        json_error("GET /subject/id")
    }
}

#[post("/subject", data = "<subject>")]
pub fn post_subject(db: super::MysqlDB, subject: Json<PostSubject>) -> JsonValue {
    let response = diesel::insert_into(subject::table)
        .values(&*subject)
        .execute(&*db);

    if let Ok(rows) = response {
        json_success(rows, "POST /subject/id")
    } else {
        json_error("POST /subject/id")
    }
}

#[put("/subject/<id>")]
pub fn put_subject(db: super::MysqlDB, id: u32) -> () {
}

#[delete("/subject/<id>")]
pub fn delete_subject(db: super::MysqlDB, id: u32) -> () {
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

fn json_success(rows: usize, path: &str) -> JsonValue {
    json!(
        {
            "rows": rows,
            "response" : "Success",
            "path": path,
        }
    )
}

fn json_error(path: &str) -> JsonValue {
    json!(
        {
            "response" : "Error completing DB request",
            "path": path,
        }
    )
}
