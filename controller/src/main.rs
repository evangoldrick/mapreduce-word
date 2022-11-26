mod routes;
#[macro_use] extern crate rocket;

use std::collections::{HashMap};

fn main_controller(state: std::sync::Arc<std::sync::Mutex<String>>) {
    let mut running = true;
    while running {
        let x = state.lock().unwrap();
        if *x == "stopping".to_string() {
            running = false;
        }
    }
    println!("mainController thread ended");
}


#[launch]
fn rocket() -> _ {
    let server_status: std::sync::Arc<std::sync::Mutex<String>> = std::sync::Arc::new(std::sync::Mutex::new("running".to_string()));
    let server_status_clone = server_status.clone();
    let x: std::thread::JoinHandle<()> = std::thread::spawn(|| {main_controller(server_status_clone)});
    rocket::build()
    .mount("/api/", routes![routes::add_words, routes::get_words, routes::end_process])
    .manage(routes::MainState {word_list_map: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())), server_status: server_status.clone(), controller_main_thread: std::sync::Arc::new(std::sync::Mutex::new(x))})
}
