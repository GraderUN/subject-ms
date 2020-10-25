use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

use super::*;
use crate::models::{SubjectGet, SubjectPost}; //, Subject};
use crate::schema::subject;

#[get("/subject")]
pub fn get_subjects(db: crate::MysqlDB) -> JsonValue {
    let response = subject::table
        .select((subject::id, subject::name, subject::grade))
        .load::<SubjectGet>(&*db);
    json_get(response, "GET /subject")
}

#[get("/subject/<id>")]
pub fn get_subject(db: crate::MysqlDB, id: u32) -> JsonValue {
    let response = subject::table
        .select((subject::id, subject::name, subject::grade))
        .find(id)
        .first::<SubjectGet>(&*db);
    json_get(response, "GET /subject/id")
}

#[get("/grade/<id>")]
pub fn get_grade(db: crate::MysqlDB, id: i32) -> JsonValue {
    let response = subject::table
        .select((subject::id, subject::name, subject::grade))
        .filter(subject::grade.eq(id))
        .load::<SubjectGet>(&*db);
    json_get(response, "GET /subject/id")
}

#[post("/subject", data = "<subject>")]
pub fn post_subject(db: crate::MysqlDB, subject: Json<SubjectPost>) -> JsonValue {
    let response = diesel::insert_into(subject::table)
        .values(&*subject)
        .execute(&*db);
    json_execute(response, "POST /subject/id")
}

#[put("/subject/<id>", data = "<subject>")]
pub fn put_subject(db: crate::MysqlDB, id: u32, subject: Json<SubjectPost>) -> JsonValue {
    let response = diesel::update(subject::table.find(id))
        .set(&*subject)
        .execute(&*db);
    json_execute(response, "PUT /subject/id")
}

#[delete("/subject/<id>")]
pub fn delete_subject(db: crate::MysqlDB, id: u32) -> JsonValue {
    let response = diesel::delete(subject::table.find(id)).execute(&*db);
    json_execute(response, "DELETE /subject/id")
}
