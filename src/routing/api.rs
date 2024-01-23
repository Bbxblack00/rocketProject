use uuid::Uuid;



#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "altro ciao!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "{}"
}