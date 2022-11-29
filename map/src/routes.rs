#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct TextJson {
    pub text: String,
    pub job_id: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JustInt {
    job_id: u32,
}

pub struct MainState {
    pub job_map: std::sync::Arc<std::sync::Mutex<std::collections::VecDeque<TextJson>>>,
    pub server_status: std::sync::Arc<std::sync::Mutex<String>>,
}

pub fn formatted_error_json(error_string: String) -> String {
    format!("{{\"error\":\"{}\"}}", error_string)
}

#[rocket::get("/words", format = "application/json", data = "<input>")]
pub fn get_words(input: String, state: &rocket::State<MainState>) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    let x: Result<JustInt, serde_json::Error> = serde_json::from_str(&input);
    match x {
        Ok(job_to_look_for) => {
            match state.job_map.lock() {
                Ok(text_map) => {
                    let mut found: (rocket::http::Status, (rocket::http::ContentType, String)) = (rocket::http::Status::InternalServerError, (rocket::http::ContentType::JSON, format!("{{\"error\":\"Job with id \\\"{}\\\" not found\"}}", job_to_look_for.job_id)));
                    for job in text_map.iter() {
                        if job.job_id == job_to_look_for.job_id {
                            found = match serde_json::to_string(&job) {
                                Ok(res) => (rocket::http::Status::Ok, (rocket::http::ContentType::JSON, res)),
                                Err(error) => (rocket::http::Status::InternalServerError, (rocket::http::ContentType::JSON, error.to_string())),
                            };
                            break;
                        }
                    }
                    found
                },
                Err(err) => panic!("Error {}", err),
            }
        },
        Err(error) => {
            eprintln!("Error {}", error);
            (rocket::http::Status::BadRequest, (rocket::http::ContentType::JSON, "{}".to_string()))
        },
    }
}

#[rocket::post("/words", format = "application/json", data = "<input>")]
pub fn add_words(input: String, state: &rocket::State<MainState>) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    let s: Result<TextJson, serde_json::Error> = serde_json::from_str(&input);
    match s {
        Ok(input_json) => {
            match state.clone().job_map.lock() {
                Ok(mut text_map) => {
                    text_map.push_back(input_json); // Add data to queue
                    (rocket::http::Status::Ok, (rocket::http::ContentType::JSON, "".to_string()))
                },
                Err(error) => {
                    eprintln!("Error {}", error);
                    (rocket::http::Status::InternalServerError, (rocket::http::ContentType::JSON, formatted_error_json(error.to_string())))
                },
            }
        },
        Err(error) => {
            eprintln!("Error {}", error);
            (rocket::http::Status::BadRequest, (rocket::http::ContentType::JSON, formatted_error_json(error.to_string())))
        },
    }
}

#[rocket::post("/endprocess")]
pub fn end_process(shut: rocket::Shutdown) -> rocket::http::Status {
    shut.notify();
    rocket::http::Status::Ok
}


