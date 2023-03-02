use core::time;
use std::sync::{Arc, RwLock};

mod routes;
use common::data_structures::{self, ControllerJob};

fn main_controller(state: Arc<RwLock<data_structures::MainStateData>>) {
    let mut running = true;
    println!("Started main_controller");
    //let mut dequeued_jobs = std::collections::VecDeque::new();

    while running {
        //println!("Heartbeat");
        {
            // Make sure that state is not always locked
            let data_state = state.read().unwrap();
            {
                let mut state_value = data_state.server_status.write().unwrap();
                if let data_structures::ServerState::StopRequested = *state_value {
                    running = false;
                    *state_value = data_structures::ServerState::Stopping;
                    //break;
                }
            }

            let mut new_jobs = data_state.new_jobs.write().unwrap();

            if !new_jobs.is_empty() {
                let mut jobs = data_state.in_progress_jobs.write().unwrap();

                let x = new_jobs.iter();
                for (key, value) in x {
                    jobs.insert(key.clone(), ControllerJob::from(value));
                    println!("Started job: {}", key)
                }
                new_jobs.clear();
            }
        }

        std::thread::sleep(time::Duration::from_millis(100));
    }

    let data_state = state.read().unwrap();
    let mut state_value = data_state
        .server_status
        .write()
        .expect("Main state not accessable");
    *state_value = data_structures::ServerState::Stopped;
    println!("Processing thread ended");
}

#[rocket::main]
async fn main() {
    common::rocket_common_main::rocket_common_main(
        rocket::routes![
            routes::new_job,
            routes::get_job,
            routes::get_jobs,
            routes::get_version,
            routes::end_process,
        ],
        main_controller,
    )
    .await;
    println!("End of main");
}
