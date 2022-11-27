mod routes;
//#[macro_use] extern crate rocket;

use std::collections::{HashMap};


fn main_controller(jobs: std::sync::Arc<std::sync::Mutex<HashMap<u32, routes::TextJson>>>, state: std::sync::Arc<std::sync::Mutex<String>>) {
    let mut running = true;
    while running {
        if *(state.lock().unwrap()) == "stopping".to_string() {
            running = false;
            *(state.lock().unwrap()) = "stopped".to_string();
        }
        match jobs.lock() {
            Ok(jobs) => {
                for (key, val) in jobs.iter() {
                    // TODO Send data to mappers

                }
            },
            Err(e) => eprint!("{:?}", e),
        }
    }
    println!("Processing thread ended");
}


#[rocket::main]
async fn main() {
    let server_status: std::sync::Arc<std::sync::Mutex<String>> = std::sync::Arc::new(std::sync::Mutex::new("running".to_string()));
    let server_status_clone = server_status.clone();
    
    let jobs1 = std::sync::Arc::new(std::sync::Mutex::new(HashMap::new()));
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
