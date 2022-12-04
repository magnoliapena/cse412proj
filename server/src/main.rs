mod utils;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::http::header;
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web,
    web::Data,
    App, HttpServer,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod api;
use api::services::{search_class, create_account};

pub struct AppState {
    db: Pool<Postgres>,
}

// const ALLOWED_ORIGIN: &str = "http://localhost:3000";

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
    std::env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "full");

    match env_logger::init() {
        Ok(_) => println!("Logger initialized"),
        Err(_) => println!("Logger could not be initialized"),
    }
    // let private_key = rand::thread_rng().gen::<[u8; 32]>();
    let private_key = Key::generate();
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database url: {}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    println!("We did a connect!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .allow_any_origin()
            .max_age(3600)
            .supports_credentials();

        let logger: Logger = Logger::default();

        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), private_key.clone())
                    .cookie_http_only(false)
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            .wrap(cors)
            .wrap(logger)
            //.service(search_class)
            .service(web::scope("/api")
                .service(search_class)
                .service(create_account)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
