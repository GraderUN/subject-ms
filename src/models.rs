#[derive(Queryable)]
pub struct Subject {
    pub id: i32,
    pub name: String,
    pub grade: i16,
    pub content: String,
}
