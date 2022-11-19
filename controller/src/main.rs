mod routes;
#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    std::thread::spawn(|| -> _ {
        println!("asd");
    });

    rocket::build()
    .mount("/", routes![routes::add_words])
    .manage(routes::mainState {wordListVector: Vec::new()})
}
