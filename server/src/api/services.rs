//usage
use actix_web::{
    get, post, web::{Data, Json, Path}, Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;

//tables
#[derive(Serialize, FromRow)] //user table
struct User{
    userid: String,
    password: String,
    location: String,
    username: String,
    major: String
}
#[derive(Serialize, FromRow)] //class table
struct Class{
    classid: i32,
    title: String,
    units: i32,
    dates: String,
    status: i32,
    days: String,
    start_time: String,
    end_time: String,
    instruction: String,
    location: String,
    course: String,
    session: String,
    term: i32
}

#[derive(Serialize, FromRow)] //class list table
struct Class_List{
    classid: String,
    semester: String
}

#[derive(Serialize, FromRow)] //wishlist table
struct Wish_List{
    userid: String,
    classid: String,
    priority_ranking: i32,
    added_date: String
}
#[derive(Serialize, FromRow)] //taken
struct Taken{
    userid: String,
    classid: String
}
#[derive(Serialize, FromRow)] //requirements
struct requirements{
    classid: String,
    prerequisites: String
}

//routes
#[derive(Deserialize)]
pub struct CreateUser{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,

}

pub struct CreateWaitList{

}

//fn
pub async fn get_user(state: Data<AppState>) -> impl Responder{
    match sqlx::query_as::<_, User>("SELECT * FROM asu_user")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_class(state: Data<AppState>) -> impl Responder{
    match sqlx::query_as::<_, Class_List>("SELECT classid from class_list")
        .fetch_all(&state.db)
        .await
    {
        Ok(class) => HttpResponse::Ok().json(class),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_waitlist(state: Data<AppState>) -> impl Responder{
    match sqlx::query_as::<_, Wish_List>("SELECT ID from waitlist")
        .fetch_all(&state.db)
        .await
    {
        Ok(waitlist) => HttpResponse::Ok().json(waitlist),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
