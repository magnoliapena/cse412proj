use actix_http::HttpMessage;
use actix_identity::Identity;
use actix_web::{web, HttpResponse, HttpRequest};
use serde::{ Serialize, Deserialize };
use deadpool_postgres::Client;
use crate::{errors::MyError, models::user};
use postgres::{Connnection, TlsMode};

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
}

/*
    curl -i --request POST \
        --url http://loclhost:4000/api/auth \
        --header 'content-type: application/json' \
        --data '{"username": "bobiscool", "password": "pass"}'
 */
pub async fn connect() -> HttpResponse {
    let url = "postgresql://postgres:cse412@localhost/postgres";
    let conn = Connection::connect(url, TlsMode:None).unwrap();
    HttpResponse::Ok().body("Connected")
}


pub async fn login(request: HttpRequest, info: web::Json<Info>) -> web::Json<User> {
    let username = info.username.clone();
    println!("[user] - login");
    println!("[user] - username: {}", username);

    //id.remember(username.to_owned());
    Identity::login(&request.extensions(), username.clone().into()).unwrap();
    web::Json(User {
        id: username,
    })
}

/*
    curl -i --request DELETE \
        --url http://localhost:4000/api/aith \
        --header 'content-type: application/json'
 */
pub async fn logout(id: Identity) -> HttpResponse {
    println!("[user] - logout");

    //id.forget();
    id.logout();
    HttpResponse::Ok().finish()
}