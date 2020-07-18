use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Result};
use api::Hello;

pub async fn hello() -> Result<HttpResponse> {
    println!("hit");
    let data = Hello::new("Hello from server");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_value(data).unwrap()))
}

pub fn route2_routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api").service(web::scope("/hello").route("", web::get().to(hello))));
}
