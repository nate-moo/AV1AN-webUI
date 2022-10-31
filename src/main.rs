#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/api/<int>")]
fn api(int: i32) -> String {
    format!("number {}", int)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, api])
}