use actix_http::HttpMessage;
use actix_identity::Identity;
use serde::{Deserialize, Serialize};

use crate::model::user;
#[allow(unused_imports)]
use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::JsonBody,
    web::Path,
    web::Query,
    HttpResponse,
    HttpRequest,
};

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
pub async fn login(request: HttpRequest, info: Json<Info>) -> Json<User> {
    let username = info.username.clone();
    println!("[user] - login");
    println!("[user] - username: {}", username);

    //id.remember(username.to_owned());
    Identity::login(&request.extensions(), username.clone().into()).unwrap();
    Json(User { id: username })
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

#[post("/api/user")]
pub async fn post_user(user_post_request: Json<user::CreateUserRequest> ) -> HttpResponse {
    println!("{:?}", user_post_request);
    HttpResponse::Ok().finish()
}
