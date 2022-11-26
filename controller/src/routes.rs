use std::collections::{HashMap};
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct TextJson {
    text: String,
    job_id: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JustInt {
    job_id: u32,
}

pub struct MainState {
    pub word_list_map: std::sync::Arc<std::sync::Mutex<HashMap<u32, TextJson>>>,
    pub server_status: std::sync::Arc<std::sync::Mutex<String>>,
    pub controller_main_thread: std::sync::Arc<std::sync::Mutex<std::thread::JoinHandle<()>>>,
}

#[get("/words", format = "application/json", data = "<input>")]
pub fn get_words(input: String, state: &rocket::State<MainState>) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    let json: JustInt = match serde_json::from_str(&input) {
        Ok(j) => j,
        Err(error) => {
            eprintln!("Error {}", error);
            return (rocket::http::Status::BadRequest, (rocket::http::ContentType::JSON, "{}".to_string()));
        },
    };

    let text_map = match state.clone().word_list_map.lock() {
        Ok(o) => o,
        Err(err) => panic!("Error {}", err),
    };

    let words = text_map.get(&json.job_id);

    return match words {
        Some(w) =>
            match serde_json::to_string(&w) {
                Ok(res) => (rocket::http::Status::Ok, (rocket::http::ContentType::JSON, res)),
                Err(error) => (rocket::http::Status::InternalServerError, (rocket::http::ContentType::JSON, error.to_string())),
            },
        None => (rocket::http::Status::BadRequest, (rocket::http::ContentType::JSON, format!("{{\"error\":\"Job with id \"{}\" does not exist\"}}", json.job_id))),
    };
}

#[post("/words", format = "application/json", data = "<input>")]
pub fn add_words(input: String, state: &rocket::State<MainState>) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    let json: TextJson = match serde_json::from_str(&input) {
        Ok(j) => j,
        Err(error) => {
            eprintln!("Error {}", error);
            return (rocket::http::Status::BadRequest, (rocket::http::ContentType::JSON, "{}".to_string()));
        },
    };

    let mut text_map = match state.clone().word_list_map.lock() {
        Ok(o) => o,
        Err(err) => panic!("Error {}", err),
    };

    let mut server_status = match state.clone().server_status.lock() {
        Ok(o) => o,
        Err(err) => panic!("Error {}", err),
    };

    let job_id = json.job_id;
    let res = text_map.insert(json.job_id, json); // Add data to queue
    return match res {
        Some(_) => (rocket::http::Status::Ok, (rocket::http::ContentType::JSON, "".to_string())),
        None => (rocket::http::Status::Conflict, (rocket::http::ContentType::JSON, format!("{{\"error\":\"Job with id \"{}\" already exists\"}}", job_id))),
    };
}

#[post("/endprocess")]
pub fn end_process(state: &rocket::State<MainState>) -> rocket::http::Status {
    match state.server_status.lock() {
        Ok(mut o) => {
            *o = "stopping".to_string();
            match state.controller_main_thread.lock() {
                Ok(thread) => {
                    let x = (&*thread).clone();
                    x.join();
                    //match {
                        //Ok(_) => rocket::http::Status::Ok,
                        //Err(_) => rocket::http::Status::InternalServerError
                    //}
                    rocket::http::Status::InternalServerError
                },
                Err(_) => rocket::http::Status::InternalServerError
            }
        },
        Err(_) => rocket::http::Status::InternalServerError
    }
}


