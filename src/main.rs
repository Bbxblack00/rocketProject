#[macro_use] extern crate rocket;
use uuid::Uuid;

let id = Uuid::new_v5();
print!(id);

#[get("/")]
fn index() -> &'static str {  // <- request handler
    "altro ciao!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
