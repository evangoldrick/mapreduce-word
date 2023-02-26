use core::time;
use std::sync::{Arc, RwLock};

mod routes;
use common::data_structures;

fn main_controller(state: Arc<RwLock<data_structures::MainStateData>>) {
    let mut running = true;
    //let mut dequeued_jobs = std::collections::VecDeque::new();

    while running {
        {
            // Make sure that state is not always locked
            let data_state = state.read().unwrap();
            let mut state_value = data_state.server_status.write().unwrap();
            if let data_structures::ServerState::StopRequested = *state_value {
                running = false;
                *state_value = data_structures::ServerState::Stopping;
                //break;
            }
        }

        std::thread::sleep(time::Duration::from_millis(10));
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
        rocket::routes![routes::new_job, routes::get_job, routes::end_process],
        main_controller,
    )
    .await;
}
