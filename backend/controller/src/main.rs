use core::time;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

mod routes;
use common::data_structures;

fn main_controller(state: Arc<Mutex<data_structures::ServerState>>) {
    let mut running = true;
    //let mut dequeued_jobs = std::collections::VecDeque::new();

    while running {
        {
            let mut state_value = state.lock().unwrap();
            if let data_structures::ServerState::StopRequested = *state_value {
                running = false;
                *state_value = data_structures::ServerState::Stopping;
                //break;
            }
            let mut jobs_vector = state_value.lock().unwrap();
        }

        std::thread::sleep(time::Duration::from_millis(10));
    }
    let mut state_value = state.lock().expect("Main state not accessable");
    *state_value = data_structures::ServerState::Stopped;
    println!("Processing thread ended");
}

#[rocket::main]
async fn main() {
    common::rocket_common_main::rocket_common_main(
        rocket::routes![routes::new_job, routes::get_job, routes::end_process],
        main_controller,
    )
    .await;
}
