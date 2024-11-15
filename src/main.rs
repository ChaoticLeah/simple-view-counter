use actix_cors::Cors;
use actix_web::{
    get, guard,
    http::{self},
    middleware::{self, Logger},
    post,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Serialize;
use std::env;
use std::{path::PathBuf, sync::Mutex};

mod config;
mod cooldown;
mod db;

#[derive(Serialize)]
struct CountResponse {
    count: i32,
}

struct AppData {
    cooldown: Mutex<cooldown::Cooldown>,
    db_path: String,
    allowed_keys: Vec<String>,
    // config: &config::Config,
}

#[post("/increment/{name}")]
// async fn increment(data: web::Path<String>) -> impl Responder {
async fn increment(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or_default();
    let data = req.app_data::<Data<Mutex<AppData>>>().unwrap();
    let connection_info = req.connection_info().clone();
    let ip = connection_info.realip_remote_addr().unwrap();
    println!("IP: {}", ip);

    let app_data = data.lock().unwrap();

    let allowed_keys = app_data.allowed_keys.clone();
    if allowed_keys.len() > 0 && !allowed_keys.contains(&name.to_string()) {
        return HttpResponse::Unauthorized().finish();
    }

    if !app_data.cooldown.lock().unwrap().check(&ip, name) {
        let views = db::get_views(name, app_data.db_path.clone());
        return HttpResponse::TooManyRequests().json(CountResponse {
            count: views.unwrap_or(0),
        });
    }

    let views = db::add_view(name, app_data.db_path.clone());
    HttpResponse::Ok().json(CountResponse {
        count: views.unwrap_or(0),
    })
}

#[get("/{name}")]
async fn get_count(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or_default();
    let data = req.app_data::<Data<Mutex<AppData>>>().unwrap();
    let app_data = data.lock().unwrap();

    let views = db::get_views(name, app_data.db_path.clone());
    return HttpResponse::Ok().json(CountResponse {
        count: views.unwrap_or(0),
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut config_path = PathBuf::from("config.yaml");
    let mut db_path = "data.db";

    // Iterate over command-line arguments
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if i + 1 < args.len() {
            if args[i] == "--config" {
                config_path = PathBuf::from(&args[i + 1]);
                break;
            }

            if args[i] == "--db" {
                db_path = &args[i + 1];
                break;
            }
        }
    }

    let config = config::load_config(config_path).await.unwrap();

    println!("Starting with Config: {:?}", config);

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

    let allowed_keys_setting = match config.allowed_keys {
        Some(allowed_keys) => allowed_keys,
        None => Vec::new(),
    };

    let log_level = match config.log_level {
        Some(log) => log,
        None => "off".to_string(),
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_level));

    let app_data: Data<Mutex<AppData>> = Data::new(Mutex::new(AppData {
        cooldown: Mutex::new(cooldown::Cooldown::new(
            cooldown_setting,
            blacklist_ips_setting,
        )),
        db_path: db_path.to_string(),
        allowed_keys: allowed_keys_setting,
    }));

    HttpServer::new(move || {
        let cors = if allowed_origin_setting == "*" {
            Cors::default().send_wildcard()
        } else {
            Cors::default().allowed_origin(&allowed_origin_setting)
        }
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

        App::new()
            .wrap(cors)
            .default_service(
                web::route().guard(guard::Options()).to(HttpResponse::Ok), // Handle preflight OPTIONS
            )
            .wrap(middleware::DefaultHeaders::new().add((
                "Content-Security-Policy",
                "default-src '*'; connect-src '*';",
            )))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::clone(&app_data))
            .service(get_count)
            .service(increment)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
