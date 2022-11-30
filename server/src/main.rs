mod api;
mod lib;
mod model;

use api::class::get_class;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, middleware::Logger, web, App, HttpResponse, HttpServer};

const ALLOWED_ORIGIN: &str = "http://localhost:3000";

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
// #[get("/users/{user_id}/{friend}")] // <- define path parameters
// async fn index(path: web::Path<(u32, String)>) -> Result<String> {
//     let (user_id, friend) = path.into_inner();
//     Ok(format!("Welcome {}, user_id {}!", friend, user_id))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let private_key = rand::thread_rng().gen::<[u8; 32]>();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(ALLOWED_ORIGIN)
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .supports_credentials();

        let logger: Logger = Logger::default();

        App::new().wrap(logger).wrap(cors).service(get_class).route(
            "/",
            web::get().to(|| async { HttpResponse::Ok().body("/") }),
        )
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
