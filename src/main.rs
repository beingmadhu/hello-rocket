#[macro_use] extern crate rocket;
use rocket::response::stream::ByteStream;
use rocket::futures::stream::{repeat, StreamExt};
use rocket::tokio::time::{self, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hi")]
fn hi() -> &'static str {
     r#"
   ---------------------------------------
<  Hello fello Rustaceans! I am Madhu :)   >        
   ---------------------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \"

    --------------------------------------
<  What is HTML ?                    ASCII >
   ---------------------------------------
_____ ______    ________   ________   ___  ___   ___  ___     
|\   _ \  _   \ |\   __  \ |\   ___ \ |\  \|\  \ |\  \|\  \    
\ \  \\\__\ \  \\ \  \|\  \\ \  \_|\ \\ \  \\\  \\ \  \\\  \   
 \ \  \\|__| \  \\ \   __  \\ \  \ \\ \\ \   __  \\ \  \\\  \  
  \ \  \    \ \  \\ \  \ \  \\ \  \_\\ \\ \  \ \  \\ \  \\\  \ 
   \ \__\    \ \__\\ \__\ \__\\ \_______\\ \__\ \__\\ \_______\
    \|__|     \|__| \|__|\|__| \|_______| \|__|\|__| \|_______|
                                                               
   "#
}

#[get("/hello")]
fn hello() -> &'static str {
    enum VectorsStuff {
        Int(i32),
        Text(String),
        Float(f64)
    }
    let _row = vec![
        VectorsStuff::Int(3),
        VectorsStuff::Text(String::from("blue")),
        //  qVectorsStuff::Float(10.12);
    ];
    let v = vec![ String::from("السلام عليكم"), 
          String::from("Dobrý den"),
          String::from("Hello"),
          String::from("שָׁלוֹם"),
          String::from("नमस्ते"),
          String::from("こんにちは"),
          String::from("안녕하세요"),
          String::from("你好"),
          String::from("Olá"),
          String::from("Здравствуйте"),
          String::from("Hola")
    ];
    let _hi_ja = String::from("こんにちは");
    "Hola"
}

#[get("/bytes")]
fn bytes() -> ByteStream![&'static [u8]] {
    ByteStream(repeat(&[1, 2, 3][..]))
}



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hi])
        .mount("/", routes![hello])
        .mount("/", routes![bytes])
        .launch()
        .await?;

    Ok(())
}
