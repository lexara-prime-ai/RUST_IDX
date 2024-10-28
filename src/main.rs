#[macro_use] extern crate rocket;

#[get("/health")]
fn health_checker() -> String {
    format!("Server is listening...")
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![health_checker])
}

