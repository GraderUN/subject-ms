use super::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Subject {
    pub id: u32,
    pub name: String,
    pub grade: Option<i32>,
    pub content: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct SubjectGet {
    pub id: u32,
    pub name: String,
    pub grade: Option<i32>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "subject"]
pub struct SubjectPost {
    pub name: String,
    pub grade: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "teacher"]
pub struct Teacher {
    pub id_teacher: u32,
    pub id_subject: u32,
}
