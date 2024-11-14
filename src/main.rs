use actix_cors::Cors;
use actix_web::{dev::ResourcePath, get, http, post, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::sync::Mutex;

mod db;
mod config;
mod cooldown;


#[derive(Serialize)]
struct CountResponse {
    count: i32,
}

#[post("/increment/{name}")]
// async fn increment(data: web::Path<String>) -> impl Responder {
async fn increment(req: HttpRequest) -> impl Responder {
    let data = req.app_data::<Data<Mutex<cooldown::Cooldown>>>().unwrap();
    let connection_info = req.connection_info().clone();
    let ip = connection_info.realip_remote_addr().unwrap();
    println!("IP: {}", ip);

    let mut cooldown = data.lock().unwrap();

    if !cooldown.check(&ip) {
        let views = db::get_views(&req.path());
        return HttpResponse::TooManyRequests().json(CountResponse { count: views.unwrap_or(0) });
    }
    let views = db::add_view(&req.path());    
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
    let allowed_origin_setting = match config.allowed_origin {
        Some(origin) => origin,
        None => "http://localhost:8080".to_string(),
    };

    let cooldown_setting = match config.cooldown {
        Some(cooldown) => cooldown,
        None => 12,
    };

    let blacklist_ips_setting = match config.blacklist_ips {
        Some(blacklist_ips) => blacklist_ips,
        None => Vec::new(),
    };
    
    let cooldown: Data<Mutex<cooldown::Cooldown>> = Data::new(Mutex::new(cooldown::Cooldown::new(cooldown_setting, blacklist_ips_setting)));
    HttpServer::new(move || {
        let allowed_origin = allowed_origin_setting.as_str();
        let cors = Cors::default()
              .allowed_origin(allowed_origin)
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::clone(&cooldown))
            .service(get_count)
            .service(increment)
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await
}
