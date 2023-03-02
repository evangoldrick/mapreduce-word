use common::{
    data_structures::{ControllerJob, JobHashMap, JobIdType, JobJson, MainState},
    rocket_common,
};
use rand::Rng;
use rocket::http::{ContentType, Status};

#[rocket::get("/job/<id>")]
pub fn get_job(id: JobIdType, state: &rocket::State<MainState>) -> (Status, (ContentType, String)) {
    let data_state = state.state.read().unwrap();
    let text_map = data_state.in_progress_jobs.read().unwrap();

    match text_map.get(&id) {
        Some(res) => rocket_common::json_ok_response(format!("{:?}", res)),
        None => rocket_common::json_response(
            Status::NotFound,
            rocket_common::formatted_error_json("Job not found".to_string()),
        ),
    }
}

#[rocket::post("/job", format = "application/json", data = "<input>")]
pub fn new_job(input: String, state: &rocket::State<MainState>) -> (Status, (ContentType, String)) {
    let json: Result<ControllerJob, serde_json::Error> = serde_json::from_str(&input);

    match json {
        Ok(input_json) => match state.state.read() {
            Ok(data) => match data.new_jobs.write() {
                Ok(mut new_jobs_map) => {
                    let mut rng = rand::thread_rng();

                    let mut insertable = false;
                    let mut new_job_num: u32 = rng.gen();
                    while !insertable {
                        match new_jobs_map.get(&new_job_num) {
                            Some(_) => {
                                new_job_num = rng.gen();
                            }
                            None => {
                                insertable = true;
                            }
                        }
                    }
                    new_jobs_map.insert(
                        new_job_num,
                        JobJson {
                            text: input_json.text,
                        },
                    );
                    rocket_common::json_accepted_response(new_job_num.to_string())
                }
                Err(error) => rocket_common::json_response(
                    Status::InternalServerError,
                    rocket_common::formatted_error_json(error.to_string()),
                ),
            },
            Err(error) => rocket_common::json_response(
                Status::InternalServerError,
                rocket_common::formatted_error_json(error.to_string()),
            ),
        },
        Err(error) => rocket_common::json_response(
            Status::InternalServerError,
            rocket_common::formatted_error_json(error.to_string()),
        ),
    }
}

#[rocket::get("/jobs")]
pub fn get_jobs(server_state: &rocket::State<MainState>) -> (Status, (ContentType, String)) {
    match server_state.state.read() {
        Ok(server_state_result) => match server_state_result.in_progress_jobs.read() {
            Ok(jobs) => {
                let temp = JobHashMap { jobs: jobs.clone() };

                match serde_json::to_string(&temp) {
                    Ok(json_as_string) => rocket_common::json_ok_response(json_as_string),
                    Err(e) => rocket_common::json_formatted_error_from_string(
                        Status::InternalServerError,
                        e.to_string(),
                    ),
                }
            }
            Err(e) => rocket_common::json_formatted_error_from_string(
                Status::InternalServerError,
                e.to_string(),
            ),
        },
        Err(e) => rocket_common::json_formatted_error_from_string(
            Status::InternalServerError,
            e.to_string(),
        ),
    }
}

#[rocket::post("/version")]
pub fn get_version() -> (Status, (ContentType, String)) {
    (Status::Ok, (ContentType::JSON, "0.1".to_string()))
}

#[rocket::post("/endprocess")]
pub fn end_process(shut: rocket::Shutdown) -> Status {
    shut.notify();
    Status::Ok
}
