use common::data_structures::{JobIdType, MainState};
use rand::Rng;

#[rocket::get("/jobs/<id>")]
pub fn get_job(
    id: JobIdType,
    state: &rocket::State<MainState>,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    let data_state = state.state.read().unwrap();
    let text_map = data_state.in_progress_jobs.read().unwrap();

    match text_map.get(&id) {
        Some(res) => common::rocket_common::json_ok_response(format!("{:?}", res)),
        None => common::rocket_common::json_response(
            rocket::http::Status::NoContent,
            common::rocket_common::formatted_error_json("Job not found".to_string()),
        ),
    }
}

#[rocket::post("/jobs", format = "application/json", data = "<input>")]
pub fn new_job(
    input: String,
    state: &rocket::State<common::data_structures::MainState>,
) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    let json: Result<common::data_structures::ControllerJob, serde_json::Error> =
        serde_json::from_str(&input);

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
                        common::data_structures::JobJson {
                            text: input_json.text,
                        },
                    );
                    common::rocket_common::json_accepted_response(new_job_num.to_string())
                }
                Err(error) => common::rocket_common::json_response(
                    rocket::http::Status::InternalServerError,
                    common::rocket_common::formatted_error_json(error.to_string()),
                ),
            },
            Err(error) => common::rocket_common::json_response(
                rocket::http::Status::InternalServerError,
                common::rocket_common::formatted_error_json(error.to_string()),
            ),
        },
        Err(error) => common::rocket_common::json_response(
            rocket::http::Status::InternalServerError,
            common::rocket_common::formatted_error_json(error.to_string()),
        ),
    }
}

#[rocket::post("/endprocess")]
pub fn end_process(shut: rocket::Shutdown) -> rocket::http::Status {
    shut.notify();
    rocket::http::Status::Ok
}
