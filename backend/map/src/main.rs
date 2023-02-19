mod routes;
//#[macro_use] extern crate rocket;
use common::data_structures;
use std::collections::VecDeque;

fn main_mapper(
    jobs: std::sync::Arc<std::sync::Mutex<VecDeque<data_structures::JobJson>>>,
    state: std::sync::Arc<std::sync::Mutex<data_structures::ServerState>>,
) {
    let mut running = true;

    while running {
        running = false;
    }
    println!("Processing thread ended");
}

#[rocket::main]
async fn main() {
    common::rocket_common_main::rocket_common_main(
        rocket::routes![routes::add_words, routes::get_words, routes::end_process],
        main_mapper,
    )
    .await;
}
