mod routes;
//#[macro_use] extern crate rocket;

use std::collections::VecDeque;

fn main_mapper(
    jobs: std::sync::Arc<std::sync::Mutex<VecDeque<common::TextJson>>>,
    state: std::sync::Arc<std::sync::Mutex<String>>,
) {
    let mut running = true;
    let mut dequeued_jobs = std::collections::VecDeque::new();

    while running {
        if *state.lock().unwrap() == "stopping".to_string() {
            running = false;
            *state.lock().unwrap() = "stopped".to_string();
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
        std::thread::sleep(core::time::Duration::from_millis(10));
    }
    println!("Processing thread ended");
}

#[rocket::main]
async fn main() {}
