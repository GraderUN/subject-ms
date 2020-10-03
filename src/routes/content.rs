use diesel::prelude::*;
use rocket::Data;
use rocket_contrib::json::JsonValue;
use std::io::Read;

use super::*;
use crate::models::*;
use crate::schema::subject;
const LIMIT: usize = 16000;

#[get("/subject/<id>/content")]
pub fn get_content(db: crate::MysqlDB, id: u32) -> JsonValue {
    let response = subject::table.find(id).first::<Subject>(&*db);
    json_get(response, "GET /subject/id/content")
}

#[put("/subject/<id>/content", data = "<data>")]
pub fn put_content(db: crate::MysqlDB, id: u32, data: Data) -> JsonValue {
    let mut stream = [0; LIMIT];

    if let Ok(n) = data.open().take(LIMIT as u64).read(&mut stream) {
        let text = String::from_utf8_lossy(&stream[0..n]);

        let response = diesel::update(subject::table.find(id))
            .set(subject::content.eq(text))
            .execute(&*db);
        json_execute(response, "PUT /subject/id/content")
    } else {
        json_error("PUT /subject/id/content")
    }
}
