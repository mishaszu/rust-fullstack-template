use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("").default_service(
                Files::new("", "./static")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
