use actix_cors::Cors;
use actix_web::{dev::ResourcePath, get, post, http, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
mod db;
mod config;


#[derive(Serialize)]
struct CountResponse {
    count: i32,
}

#[post("/increment/{name}")]
async fn increment(data: web::Path<String>) -> impl Responder {
    let views = db::add_view(&data.path());    
    HttpResponse::Ok().json(CountResponse { count: views.unwrap_or(0) })
}

#[get("/{name}")]
async fn get_count(data: web::Path<String>) -> impl Responder {
    let views = db::get_views(&data.path());
    HttpResponse::Ok().json(CountResponse { count: views.unwrap_or(0) })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::load_config().await.unwrap();
    let allowed_origin_string = match config.allowed_origin {
        Some(origin) => origin,
        None => "http://localhost:8080".to_string(),
    };
    
    HttpServer::new(move || {
        let allowed_origin = allowed_origin_string.as_str();
        let cors = Cors::default()
              .allowed_origin(allowed_origin)
              //.allowed_origin_fn(|origin, _req_head| {
              //    origin.as_bytes().ends_with(b".leahdevs.xyz/")
              //})
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_count)
            .service(increment)
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}
