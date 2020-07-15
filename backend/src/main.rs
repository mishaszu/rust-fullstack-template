use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder, Result};
use api::Hello;
use serde_json;
use std::env;

#[get("/api/hello")]
pub async fn index() -> Result<HttpResponse> {
    let data = Hello::new("Hello from server");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_value(data).unwrap()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let static_path = match args.get(1) {
        Some(value) if value == "workspace" => "./frontend/public",
        _ => "./static",
    };
    HttpServer::new(move || {
        App::new().service(index).service(
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
