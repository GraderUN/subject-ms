use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

use super::*;
use crate::models::*;
use crate::schema::teacher;

#[get("/subject/<id>/teacher")]
pub fn get_subject_teachers(db: crate::MysqlDB, id: u32) -> JsonValue {
    let response = teacher::table
        .filter(teacher::id_subject.eq(id))
        .load::<Teacher>(&*db);
    json_get(response, "GET /subject/id/teacher")
}

#[get("/teacher/<id>")]
pub fn get_teacher_subjects(db: crate::MysqlDB, id: u32) -> JsonValue {
    let response = teacher::table
        .filter(teacher::id_teacher.eq(id))
        .load::<Teacher>(&*db);
    json_get(response, "GET /teacher/id")
}

#[post("/teacher", data = "<data>")]
pub fn post_teacher(db: crate::MysqlDB, data: Json<Teacher>) -> JsonValue {
    let response = diesel::insert_into(teacher::table)
        .values(&*data)
        .execute(&*db);
    json_execute(response, "POST /teacher")
}

#[delete("/teacher?<id_teacher>&<id_subject>")] // , data = "<data>")]
pub fn delete_teacher(db: crate::MysqlDB, id_teacher: u32, id_subject: u32) -> JsonValue {
    let response = diesel::delete(teacher::table.find((id_teacher, id_subject))).execute(&*db);
    json_execute(response, "DELETE /teacher")
}
