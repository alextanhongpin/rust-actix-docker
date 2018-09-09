extern crate actix;
extern crate actix_web;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;

use actix_web::{http, server, App, Path, Responder, HttpResponse, Json};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Greet {
    name: String,
    number: i32,
}

fn hello(item: Json<Greet>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0)
}

fn index(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id: {}", info.1, info.0)
}

fn main() {
    let addr = match env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(_e) => "0.0.0.0:8000".to_string(),
    };
    let sys = actix::System::new("json-example");

    server::new(|| {
        App::new()
            .resource("/hello", |r| {
                r.method(http::Method::POST)
                    .with_config(hello, |(cfg,)| {
                        cfg.limit(4096); // Limit size of the payload.
                    })
            })
        .route("/{id}/{name}/index.html", http::Method::GET, index)
    })
    .bind(&addr)
    .unwrap()
    .shutdown_timeout(1)
    .start();

    println!("Started http server: {}", &addr);
    let _ = sys.run();
}
