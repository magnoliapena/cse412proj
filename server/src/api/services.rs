//usage
use crate::AppState;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

//schemas
#[derive(Serialize, FromRow)] //user table
struct User {
    userid: String, //pk
    password: String,
    location: String,
    username: String,
    major: String,
}

#[derive(Serialize, FromRow)] //class table (contains all classes at ASU)
struct Class {
    classid: i32, //pk
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
    term: String,
}

#[derive(Serialize, FromRow)] //class list table (sub table of classes for a user in wishlist)
struct ClassList {
    //hidden table from user
    class_id: String,   //pk
    class_list: String, //array of classes
    semester: String,   //semester of the class list of user
}

#[derive(Serialize, FromRow)] //wishlist table
struct WishList {
    user_id: String,
    class_list_id: String,
    priority_ranking: i32,
    added_date: String,
}

#[derive(Serialize, FromRow)] //taken
struct Taken {
    userid: String,   //references asu_user table
    class_id: String, //references class
}

#[derive(Serialize, FromRow)] //requirements
struct Requirements {
    class_id: String,      //references class
    prerequisites: String, //text array
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub userid: i32,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
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

//USER GET REQUESTS
#[get("/user/{userid}")] //get single user from id
pub async fn get_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match sqlx::query_as::<_, User>("SELECT * FROM asu_user WHERE userid = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("User doesn't exist"),
    }
}

#[get("/user/{userid}/wishlist")] //get entire class list of a user
pub async fn get_classlist(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let userid: i32 = path.into_inner();
    match sqlx::query_as::<_, WishList>(
        "SELECT * from class_list, wishlist\
    WHERE wishlist.userid = $1 and wishlist.classlistid = class_list.classlistid",
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(wishlist) => HttpResponse::Ok().json(wishlist),
        Err(_) => HttpResponse::NotFound().json("Wishlist doesn't exist"),
    }
}

//CLASS GET REQUESTS
#[get("/class/{classid}")] //get single class from class id
pub async fn get_class(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let classid: i32 = path.into_inner();
    match sqlx::query_as::<_, Class>("SELECT classid from class_list where classid = $1")
        .bind(classid)
        .fetch_all(&state.db)
        .await
    {
        Ok(class) => HttpResponse::Ok().json(class),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
//might need to add more depending if we use filters (if i need to make a request for specific filter)
//idk if we're implementing that
#[get("/classes")] //get all classes //general template
pub async fn get_all_classes(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Class>("SELECT * FROM class")
        .fetch_all(&state.db)
        .await
    {
        Ok(classes) => HttpResponse::Ok().json(classes),
        Err(e) => HttpResponse::NotFound().json("No classes inputted into data"),
    }
}
#[get("/classes/{term}")] // list all classes for a term
pub async fn get_classes_filterby_term(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Class>("SELECT * FROM class WHERE term = $1")
        .fetch_all(&state.db)
        .await
    {
        Ok(classes) => HttpResponse::Ok().json(classes),
        Err(_) => HttpResponse::NotFound().json("No classes inputted into data"),
    }
}

#[get("/class/{classid}/prerequisites")]
//get the requirements of a single class based off classid
pub async fn get_requirements(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let classid: i32 = path.into_inner();
    match sqlx::query_as::<_, Requirements>(
        "SELECT prerequisites from requirements WHERE classid = $1",
    )
    .bind(classid)
    .fetch_all(&state.db)
    .await
    {
        Ok(prerequisites) => HttpResponse::Ok().json(prerequisites),
        Err(_) => HttpResponse::NotFound().json("Requirements or class doesn't exist"),
    }
}

//post functions
#[post("/user/{userid}")] //post user
pub async fn post_user(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateUser>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, User>(
        "INSERT INTO asu_user (password, location, username, major)\
            VALUES ($1, $2, $3, $4, $5) RETURNING userid, password, location, username, major",
    )
    .bind(body.userid.to_string())
    .bind(body.password.to_string())
    .bind(body.location.to_string())
    .bind(body.username.to_string())
    .bind(body.major.to_string())
    .bind(id)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

#[post("/user/{userid}/wishlist")] //post wishlist
pub async fn post_wishlist(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateWishList>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, WishList>(
        "INSERT INTO wishlist (userid, classlistid, priority_ranking, added_date)\
         VALUES($1, $2, $3, $4) RETURNING userid, classlistid, priority_ranking, added_date",
    )
    .bind(body.userid.to_string())
    .bind(body.classlistid.to_string())
    .bind(body.priority_ranking.to_string())
    .bind(body.added_date.to_string())
    .bind(id)
    .fetch_one(&state.db)
    .await
    {
        Ok(wishlist) => HttpResponse::Ok().json(wishlist),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create wishlist"),
    }
}

#[derive(Deserialize)]
struct ClassInfoRequest {
    // info for class search
    class_name: String,
    class_cat: String,
    class_instructor: String,
}

#[derive(Serialize, FromRow, Debug)]
struct ClassInfo {
    title: String,
    course: String,
    instructor: String,
}


#[get("/search_class")]
pub async fn search_class(state: Data<AppState>, body: Json<ClassInfoRequest>) -> impl Responder {
    match sqlx::query_as::<_, ClassInfo>(
        "SELECT title, course, instructor FROM class WHERE course = $1"
    )
    .bind(body.class_cat.to_string())
    .fetch_all(&state.db)
    .await {
        Ok(class_search_results) => HttpResponse::Ok().json(class_search_results),
        Err(_) => HttpResponse::NotFound().json("No classes found"),
    }
}
