#[macro_use] extern crate rocket;
use std::vec;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct WordList {
    text: String
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Other {
    text: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/words", format = "application/json", data = "<input>")]
fn add_words(input: String) -> (rocket::http::Status, (rocket::http::ContentType, String)) {
    println!("Recieved: {}", input);
    let json: WordList = match serde_json::from_str(&input) {
        Ok(j) => j,
        Err(error) => {
            println!("Error {}", error);
            return (rocket::http::Status::BadRequest, (rocket::http::ContentType::JSON, "{}".to_string()));
        },
    };

    let response: WordList = WordList { text: "test response".to_string() };
    println!("Des: {:?}", json);
    
    return match serde_json::to_string(&response) {
        Ok(r) => (rocket::http::Status::Ok, (rocket::http::ContentType::JSON, r)),
        Err(error) => (rocket::http::Status::InternalServerError, (rocket::http::ContentType::JSON, error.to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    std::thread::spawn(|| -> _ {
        println!("asd");
    });
    rocket::build().mount("/", routes![index, add_words])
}