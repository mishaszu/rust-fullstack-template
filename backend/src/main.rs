use actix_files;
use actix_web::{middleware, web, App, HttpServer, Result};
use std::env;
mod route1;
mod route2;

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
            .configure(route1::route1_routes)
            .configure(route2::route2_routes)
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
