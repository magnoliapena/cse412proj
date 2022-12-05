use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(FromRow, Deserialize, Serialize)]
pub struct GetClassListId {
    pub classlistid: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct AddToWishlist {
    pub userid: String,
    pub classid: i32,
    pub term: i32,
}

#[derive(Deserialize, Serialize)] //user table
pub struct LoginInfo {
    pub userid: String, //pk
    pub password: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct LoginResponse {
    pub username: String,
    pub userid: String,
    pub location: String,
    pub major: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub password: String,
    pub username: String,
    pub email: String,
    pub location: String,
    pub major: String,
}

#[derive(Deserialize)]
pub struct CreateWishList {
    pub userid: i32,
    pub classlistid: i32,
    pub priority_ranking: i32,
    pub added_date: String,
}

#[derive(Serialize, FromRow)] //user table
pub struct User {
    pub userid: String, //pk
    pub password: String,
    pub location: String,
    pub username: String,
    pub major: String,
}

#[derive(Serialize, FromRow)] //class table (contains all classes at ASU)
pub struct Class {
    pub classid: i32, //pk
    pub title: String,
    pub units: i32,
    pub dates: String,
    pub status: i32,
    pub days: String,
    pub start_time: String,
    pub end_time: String,
    pub instruction: String,
    pub location: String,
    pub course: String,
    pub session: String,
    pub term: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct ClassInfo {
    pub classid: i32,
    pub title: String,
    pub units: i32,
    pub dates: String,
    pub status: i32,
    pub days: String,
    pub starttime: String,
    pub endtime: String,
    pub instructor: Vec<String>,
    pub location: String,
    pub course: String,
    pub session: String,
    pub term: i32,
}

#[derive(Serialize, FromRow)] //class list table (sub table of classes for a user in wishlist)
pub struct ClassList {
    //hidden table from user
    pub class_id: String,   //pk
    pub class_list: String, //array of classes
    pub semester: String,   //semester of the class list of user
}

#[derive(Serialize, FromRow)] //wishlist table
pub struct WishList {
    pub user_id: String,
    pub class_list_id: String,
    pub priority_ranking: i32,
    pub added_date: String,
}

#[derive(Serialize, FromRow)] //taken
pub struct Taken {
    pub userid: String,   //references asu_user table
    pub class_id: String, //references class
}

#[derive(Serialize, FromRow)] //requirements
pub struct Requirements {
    pub class_id: String,      //references class
    pub prerequisites: String, //text array
}
