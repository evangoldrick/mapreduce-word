mod routes;
//#[macro_use] extern crate rocket;

use core::time;
use std::collections::VecDeque;

fn main_controller(
    jobs: std::sync::Arc<std::sync::Mutex<VecDeque<routes::TextJson>>>,
    state: std::sync::Arc<std::sync::Mutex<common::ServerStates>>,
) {
    let mut running = true;
    let mut dequeued_jobs = std::collections::VecDeque::new();

    while running {
        let mut state_value = state.lock().expect("Main state not accessable");
        match *state_value {
            common::ServerStates::StopRequested => {
                running = false;
                *state_value = common::ServerStates::Stopping;
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
    *state_value = common::ServerStates::Stopped;
    println!("Processing thread ended");
}

#[rocket::main]
async fn main() {
    let server_status: std::sync::Arc<std::sync::Mutex<common::ServerStates>> =
        std::sync::Arc::new(std::sync::Mutex::new(common::ServerStates::Running));
    let server_status_clone = server_status.clone();

    let jobs1 = std::sync::Arc::new(std::sync::Mutex::new(VecDeque::new()));
    let jobs2 = jobs1.clone();

    let proccessing_thread = std::thread::spawn(|| main_controller(jobs1, server_status_clone));

    let rocket_server = rocket::build()
        .mount(
            "/api/",
            rocket::routes![routes::add_words, routes::get_words, routes::end_process],
        )
        .manage(routes::MainState {
            job_map: jobs2,
            server_status: server_status.clone(),
        });

    match rocket_server.launch().await {
        Ok(_) => println!("REST server closed"),
        Err(e) => eprintln!("{:?}", e),
    }
    {
        let x = server_status.lock();
        match x {
            Err(e) => eprintln!("{:?}", e),
            Ok(mut status) => {
                *status = common::ServerStates::StopRequested;
                println!("Waiting for processing thread to finish");
            }
        }
    }
    match proccessing_thread.join() {
        Err(e) => eprintln!("{:?}", e),
        Ok(_) => println!("Processing thread joined"),
    }
}
