use crate::data_structures;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub async fn rocket_common_main(
    routes: Vec<rocket::Route>,
    processing_thread: fn(
        jobs: Arc<Mutex<VecDeque<data_structures::TextJson>>>,
        state: Arc<Mutex<data_structures::ServerState>>,
    ),
) {
    let server_state: Arc<Mutex<data_structures::ServerState>> =
        Arc::new(Mutex::new(data_structures::ServerState::Running));

    let jobs = Arc::new(Mutex::new(VecDeque::new()));

    let server_state_clone = server_state.clone();
    let jobs_clone = jobs.clone();
    let proccessing_thread =
        std::thread::spawn(move || processing_thread(jobs_clone, server_state_clone));

    let rocket_server = rocket::build()
        .mount("/api/", routes)
        .manage(data_structures::MainState {
            job_map: jobs.clone(),
            server_status: server_state.clone(),
        });

    match rocket_server.launch().await {
        Ok(_) => println!("REST server closed"),
        Err(e) => eprintln!("{:?}", e),
    }
    {
        let x = server_state.lock();
        match x {
            Err(e) => eprintln!("{:?}", e),
            Ok(mut status) => {
                *status = data_structures::ServerState::StopRequested;
                println!("Waiting for processing thread to finish");
            }
        }
    }
    match proccessing_thread.join() {
        Err(e) => eprintln!("{:?}", e),
        Ok(_) => println!("Processing thread joined"),
    }
}
