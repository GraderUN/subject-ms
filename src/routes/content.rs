use diesel::prelude::*;
use rocket_contrib::json::JsonValue;

use super::*;
use crate::models::*;
use crate::schema::subject;

#[get("/subject/<id>/content")]
pub fn get_content(db: crate::MysqlDB, id: u32) -> JsonValue {
    let response = subject::table.find(id).first::<Subject>(&*db);
    json_get(response, "GET /subject/id/content")
}

#[put("/subject/<id>/content", data = "<text>")]
pub fn put_content(db: crate::MysqlDB, id: u32, text: String) -> JsonValue {
    let response = diesel::update(subject::table.find(id))
        .set(subject::content.eq(text))
        .execute(&*db);
    json_execute(response, "PUT /subject/id/content")
}
