use actix_files;
use actix_web::http::StatusCode;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Result};
use api::Hello;
use serde_json;
use std::env;

#[get("/api/hello")]
pub async fn hello() -> Result<HttpResponse> {
    let data = Hello::new("Hello from server");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_value(data).unwrap()))
}

pub async fn index() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./static/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let static_path = match args.get(1) {
        Some(value) if value == "workspace" => "./frontend/public",
        _ => "./static",
    };
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(
                actix_files::Files::new("/", static_path)
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            .default_service(web::resource("").route(web::get().to(index)))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
