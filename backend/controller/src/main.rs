use core::time;
use std::collections::VecDeque;

mod routes;
use common::data_structures;

fn main_controller(
    jobs: std::sync::Arc<std::sync::Mutex<VecDeque<data_structures::TextJson>>>,
    state: std::sync::Arc<std::sync::Mutex<data_structures::ServerState>>,
) {
    let mut running = true;
    let mut dequeued_jobs = std::collections::VecDeque::new();

    while running {
        let mut state_value = state.lock().expect("Main state not accessable");
        match *state_value {
            data_structures::ServerState::StopRequested => {
                running = false;
                *state_value = data_structures::ServerState::Stopping;
            }
            _ => {}
        }
        match jobs.lock() {
            Ok(mut jobs) => {
                // TODO Send data to mappers
                while !jobs.is_empty() {
                    dequeued_jobs
                        .push_back(jobs.pop_front().expect("Tried to pop from empty queue"));
                }
            }
            Err(e) => eprint!("{:?}", e),
        }

        for j in dequeued_jobs.iter() {
            println!("Completed job {}: {}", j.job_id, j.text);
        }

        dequeued_jobs.clear();
        std::thread::sleep(time::Duration::from_millis(10));
    }
    let mut state_value = state.lock().expect("Main state not accessable");
    *state_value = data_structures::ServerState::Stopped;
    println!("Processing thread ended");
}

#[rocket::main]
async fn main() {
    common::rocket_common_main::rocket_common_main(
        rocket::routes![routes::add_words, routes::get_words, routes::end_process],
        main_controller,
    )
    .await;
}
