use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Result};
use api::Hello;

#[get("")]
pub async fn index() -> Result<HttpResponse> {
    let data = Hello::new("This text is generated via backend");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_value(data).unwrap()))
}

pub fn route1_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/test").service(index));
}
