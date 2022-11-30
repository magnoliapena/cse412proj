mod lib;
mod api;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_web::{ middleware, web, web::Data, App, HttpServer, HttpResponse, cookie::{ Key, SameSite } };
use actix_session::{ SessionMiddleware, storage::CookieSessionStore };
use actix_web::http::header;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState{
    db: Pool<Postgres>
}

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
    let private_key = Key::generate();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(ALLOWED_ORIGIN)
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .supports_credentials()
            .app_data(Data::new(AppState{db: pool.clone()}));

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::builder(
                CookieSessionStore::default(),
                private_key.clone()
            )
                .cookie_http_only(false)
                .cookie_same_site(SameSite::Lax)
                .build(),
            )
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(api::user::login))
                            .route(web::delete().to(api::user::logout))
                    )
                    .service(
                        web::scope("/main")
                            .service(
                                web::resource("/all")
                                    .route(web::get().to(api::class_list::get_all))
                            )
                    )
                    .route("/", web::get().to(|| async { HttpResponse::Ok().body("api") }))
            )
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("/") } ))
    })
        .bind("127.0.0.1:4000")?
        .run()
        .await
}