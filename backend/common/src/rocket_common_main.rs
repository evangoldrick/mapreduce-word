use crate::data_structures;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub async fn rocket_common_main(
    routes: Vec<rocket::Route>,
    processing_thread: fn(state: Arc<RwLock<data_structures::MainStateData>>),
) {
    let main_state = data_structures::MainState {
        state: Arc::new(RwLock::new(data_structures::MainStateData {
            new_jobs: Arc::new(RwLock::new(HashMap::new())),
            in_progress_jobs: Arc::new(RwLock::new(HashMap::new())),
            server_status: Arc::new(RwLock::new(data_structures::ServerState::Running)),
        })),
    };

    let server_state_clone = main_state.state.clone();
    let server_state_clone2 = main_state.state.clone();

    let proccessing_thread = std::thread::spawn(move || processing_thread(server_state_clone));

    let rocket_server = rocket::build().mount("/api/", routes).manage(main_state);

    match rocket_server.launch().await {
        Ok(_) => println!("REST server closed"),
        Err(e) => eprintln!("{:?}", e),
    }

    // Request processing thread to stop
    {
        //let x = ;
        match server_state_clone2.read().unwrap().server_status.write() {
            Ok(mut status) => match *status {
                data_structures::ServerState::Running => {
                    *status = data_structures::ServerState::StopRequested;
                    println!("Waiting for processing thread to finish");
                }
                data_structures::ServerState::StopRequested
                | data_structures::ServerState::Stopping => {
                    println!("Waiting for processing thread to finish");
                }
                data_structures::ServerState::Stopped => {
                    println!("Processing thread already stopped");
                }
            },
            Err(e) => eprintln!("{:?}", e),
        };
    }

    match proccessing_thread.join() {
        Err(e) => eprintln!("{:?}", e),
        Ok(_) => println!("Processing thread joined"),
    }
}
