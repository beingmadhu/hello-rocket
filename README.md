## This is how I rocket 

```bash
cargo new hello-rocket --bin
cd hello-rocket

## Add rocket -- apparently this did not add latest automagically.
cargo add rocket

## Update it manually. 
```
vi Cargo.toml
```

#### `Cargo.toml` looks like this:
```toml
[package]
name = "hello-rocket"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rocket = "0.5.0-rc.2"
```

--- 

```bash
cargo update
```
Make sure stuff is updated.

## Added rocket code:
```bash
vi src/main.rs
```

#### `main.rs` looks like this: 
```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```

---

## Yay Rocket: 
```bash 
cargo run
```

Very simple looks good: 

```bash
📬 Routes:
   >> (index) GET /
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> X-Content-Type-Options: nosniff
   >> X-Frame-Options: SAMEORIGIN
   >> Permissions-Policy: interest-cohort=()
🚀 Rocket has launched from http://127.0.0.1:8000
```


## Time to test:
Looks like its working: 
```bash
(base) MacBook-Air:~ Madhu$ curl -vv http://127.0.0.1:8000/

*   Trying 127.0.0.1:8000...
* Connected to 127.0.0.1 (127.0.0.1) port 8000 (#0)
> GET / HTTP/1.1
> Host: 127.0.0.1:8000
> User-Agent: curl/7.82.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-type: text/plain; charset=utf-8
< server: Rocket
< permissions-policy: interest-cohort=()
< x-frame-options: SAMEORIGIN
< x-content-type-options: nosniff
< content-length: 13
< date: Mon, 04 Jul 2022 16:26:47 GMT
<
* Connection #0 to host 127.0.0.1 left intact
Hello, world!

```


## Running on my Macbook Air ~:) 
```bash
MacBook-Air:hello-rocket Madhu$
```
