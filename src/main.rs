#[macro_use]
extern crate rocket;

mod models;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Tanaka oc cho"
}
