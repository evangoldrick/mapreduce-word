use crate::data_structures::StandardErrorString;

pub fn formatted_error_json(error_string: String) -> String {
    let x = StandardErrorString { text: error_string };
    match serde_json::to_string(&x) {
        Ok(o) => o,
        Err(_) => "Failed to create error string".to_string(),
    }
}

pub fn json_response(
    status: rocket::http::Status,
    message: String,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    (status, (rocket::http::ContentType::JSON, message))
}

pub fn json_ok_response(
    json_string: String,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    json_response(rocket::http::Status::Ok, json_string)
}

pub fn json_accepted_response(
    json_string: String,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    json_response(rocket::http::Status::Accepted, json_string)
}

pub fn json_internal_server_error_response(
    json_string: String,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    json_response(rocket::http::Status::InternalServerError, json_string)
}

pub fn json_formatted_error_from_string(
    status: rocket::http::Status,
    json_string: String,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    json_response(status, formatted_error_json(json_string))
}
