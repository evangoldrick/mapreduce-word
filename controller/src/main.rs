mod routes;
//#[macro_use] extern crate rocket;

use core::time;
use std::collections::{VecDeque};

fn main_controller(jobs: std::sync::Arc<std::sync::Mutex<VecDeque<routes::TextJson>>>, state: std::sync::Arc<std::sync::Mutex<String>>) {
    let mut running = true;
    let mut dequeued_jobs = std::collections::VecDeque::new();

    while running {
        if *(state.lock().unwrap()) == "stopping".to_string() {
            running = false;
            *(state.lock().unwrap()) = "stopped".to_string();
        }
        match jobs.lock() {
            Ok(mut jobs) => {
                // TODO Send data to mappers
                while !jobs.is_empty() {
                    dequeued_jobs.push_back(jobs.pop_front().expect("Tried to pop from empty queue"));
                }
            },
            Err(e) => eprint!("{:?}", e),
        }

        for j in dequeued_jobs.iter() {
            println!("Completed job {}: {}", j.job_id, j.text);
        }

        dequeued_jobs.clear();
        std::thread::sleep(time::Duration::from_millis(10));
    }
    println!("Processing thread ended");
}


#[rocket::main]
async fn main() {
    let server_status: std::sync::Arc<std::sync::Mutex<String>> = std::sync::Arc::new(std::sync::Mutex::new("running".to_string()));
    let server_status_clone = server_status.clone();
    
    let jobs1 = std::sync::Arc::new(std::sync::Mutex::new(VecDeque::new()));
    let jobs2 = jobs1.clone();

    let proccessing_thread = std::thread::spawn(|| {main_controller(jobs1, server_status_clone)});

    let rocket_server = rocket::build()
    .mount("/api/", rocket::routes![routes::add_words, routes::get_words, routes::end_process])
    .manage(routes::MainState {job_map: jobs2, server_status: server_status.clone()});

    match rocket_server.launch().await {
        Ok(_) => println!("REST server closed"),
        Err(e) => eprintln!("{:?}", e),
    };

    *(server_status.lock().expect("Could not lock sever status variable")) = "stopping".to_string();
    println!("Waiting for processing thread to finish");
    match proccessing_thread.join() {
        Ok(_) => println!("Processing thread joined"),
        Err(e) => eprintln!("{:?}", e),
    };
}
