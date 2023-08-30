mod error;

use crate::error::SerializerError;
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::SimpleInputFunctionBuilder;
use actix_extensible_rate_limit::RateLimiter;
use actix_web::middleware::Logger;
use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use corn::Value;
use std::time::Duration;
use std::{env, io};

#[post("/parse")]
async fn parse(req: HttpRequest, body: String) -> impl Responder {
    match corn::parse(&body) {
        Ok(value) => {
            let ty = req
                .headers()
                .get("accept")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("application/json");

            match get_value_string(&value, ty) {
                Ok(str) => HttpResponse::Ok().body(str),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

fn get_value_string(value: &Value, ty: &str) -> Result<String, SerializerError> {
    match ty {
        "application/toml" => toml::to_string(value).map_err(SerializerError::from),
        "application/yaml" => serde_yaml::to_string(value).map_err(SerializerError::from),
        _ => serde_json::to_string(value).map_err(SerializerError::from),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(5050);

    let requests_per_min = env::var("REQUESTS_PER_MINUTE")
        .ok()
        .and_then(|rpm| rpm.parse().ok())
        .unwrap_or(20);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Starting server on http://{host}:{port}");
    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), requests_per_min)
            .real_ip_key()
            .build();

        let ratelimit_backend = InMemoryBackend::builder().build();
        let rate_limiter = RateLimiter::builder(ratelimit_backend.clone(), input)
            .add_headers()
            .build();

        App::new()
            .wrap(rate_limiter)
            .wrap(Logger::default())
            .service(parse)
    })
    .bind((host, port))?
    .run()
    .await
}
