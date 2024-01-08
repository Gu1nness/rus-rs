use rocket::serde::json::{json, Json, Value};

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({"status": "notok", "reason": "NotFound"}))
}

pub fn already_present() -> Json<Value> {
    Json(json![{"status": "notok", "reason": "AlreadyPresent"}])
}
