use serde::{Serialize, Deserialize};
use super::schema::*;

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
#[table_name="subject"]
pub struct SubjectPost {
    pub name: String,
    pub grade: i32,
}
