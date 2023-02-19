use common::data_structures::{JobIdType, MainState};

pub fn formatted_error_json(error_string: String) -> String {
    format!("{{\"error\":\"{}\"}}", error_string)
}

#[rocket::get("/jobs/<id>")]
pub fn get_job(
    id: JobIdType,
    state: &rocket::State<MainState>,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    let mut found: (rocket::http::Status, (rocket::http::ContentType, String)) = (
        rocket::http::Status::InternalServerError,
        (
            rocket::http::ContentType::JSON,
            format!("{{\"error\":\"Job with id \\\"{}\\\" not found\"}}", id),
        ),
    );

    let text_map = state.in_progress_jobs.lock().unwrap();

    match text_map.get(&id) {
        Some(res) => (
            rocket::http::Status::Ok,
            (rocket::http::ContentType::JSON, format!("{:?}", res)),
        ),
        None => (
            rocket::http::Status::InternalServerError,
            (
                rocket::http::ContentType::JSON,
                formatted_error_json("Job not found".to_string()),
            ),
        ),
    }
}

#[rocket::post("/jobs", format = "application/json", data = "<input>")]
pub fn new_job(
    input: String,
    state: &rocket::State<common::data_structures::MainState>,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    let json: Result<common::data_structures::TextJson, serde_json::Error> =
        serde_json::from_str(&input);

    match json {
        Ok(input_json) => {
            match state.new_jobs.lock() {
                Ok(mut text_map) => {
                    text_map.push_back(input_json); // Add data to queue
                    (
                        rocket::http::Status::Accepted,
                        (rocket::http::ContentType::JSON, "".to_string()),
                    )
                }
                Err(error) => {
                    eprintln!("Error {}", error);
                    (
                        rocket::http::Status::InternalServerError,
                        (
                            rocket::http::ContentType::JSON,
                            formatted_error_json(error.to_string()),
                        ),
                    )
                }
            }
        }
        Err(error) => {
            eprintln!("Error {}", error);
            (
                rocket::http::Status::BadRequest,
                (
                    rocket::http::ContentType::JSON,
                    formatted_error_json(error.to_string()),
                ),
            )
        }
    }
}

#[rocket::post("/endprocess")]
pub fn end_process(shut: rocket::Shutdown) -> rocket::http::Status {
    shut.notify();
    rocket::http::Status::Ok
}
