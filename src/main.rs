#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hi")]
fn hi() -> &'static str {
    r#"
    Hello fello Rustaceans! I am Madhu :)        
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \"#
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hi])
        .launch()
        .await?;

    Ok(())
}
