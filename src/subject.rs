/*
subject.rs

The Subject microservice relates all information of a particular subject within Grader.

Each subject has select details.
A subject may have a detailed content.
A subject may be taught by several teachers.
*/

#[get("/subjects")]
pub fn get_subjects() -> &'static str {
    "The get subjects path"
}

#[get("/subject")]
pub fn get_subject() -> &'static str {
    "The get subject path"
}

#[post("/subject")]
pub fn post_subject() -> &'static str {
    "The post subject path"
}

#[put("/subject")]
pub fn put_subject() -> &'static str {
    "The put subject path"
}

#[delete("/subject")]
pub fn delete_subject() -> &'static str {
    "The delete subject path"
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
