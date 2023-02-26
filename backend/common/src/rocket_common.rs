pub fn formatted_error_json(error_string: String) -> String {
    format!("{{\"error\":\"{}\"}}", error_string)
}

pub fn json_response(
    status: rocket::http::Status,
    message: String,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    (status, (rocket::http::ContentType::JSON, message))
}

pub fn json_ok_response(j: String) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    json_response(rocket::http::Status::Ok, j)
}

pub fn json_accepted_response(
    j: String,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    json_response(rocket::http::Status::Accepted, j)
}
