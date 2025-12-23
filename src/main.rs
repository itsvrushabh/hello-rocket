#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! Testing Rocket framework."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
