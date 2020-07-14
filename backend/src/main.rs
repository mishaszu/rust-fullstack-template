use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let static_path = match args.get(1) {
        Some(value) if value == "workspace" => "./frontend/public",
        _ => "./static",
    };
    println!("{}", static_path);
    HttpServer::new(move || {
        App::new().service(
            web::scope("").default_service(
                Files::new("", static_path)
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
