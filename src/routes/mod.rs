use rocket_contrib::json::JsonValue;
use serde::Serialize;

pub mod content;
pub mod subject;
pub mod teacher;

fn json_success(rows: usize, path: &str) -> JsonValue {
    json!({
        "rows": rows,
        "response" : "Success",
        "path": path,
    })
}

fn json_error(path: &str) -> JsonValue {
    json!({
        "response" : "Error completing DB request",
        "path": path,
    })
}

fn json_get<T: Serialize>(rspns: Result<T, diesel::result::Error>, path: &str) -> JsonValue {
    // T can be GetResult or Vec<GetResult>
    if let Ok(rows) = rspns {
        json!(rows)
    } else {
        json_error(path)
    }
}

fn json_execute(rspns: Result<usize, diesel::result::Error>, path: &str) -> JsonValue {
    // usize indicates the number of affected rows in the table
    if let Ok(rows) = rspns {
        json_success(rows, path)
    } else {
        json_error(path)
    }
}
