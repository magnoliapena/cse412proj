use std::collections::HashMap;
use std::error::Error;
//usage
use crate::AppState;
use crate::api::class_list::create_classlist_for_user;
use actix_web::{
    get, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Responder,
};
use regex::Regex;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use uuid::Uuid;


//schemas
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
pub struct CreateWishList {
    pub userid: i32,
    pub classlistid: i32,
    pub priority_ranking: i32,
    pub added_date: String,
}

//USER GET REQUESTS
#[get("/user/{userid}")] //get single user from id
pub async fn get_user(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let id: String = path.into_inner();
    match sqlx::query_as::<_, User>("SELECT * FROM asu_user WHERE userid = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("User doesn't exist"),
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub password: String,
    pub username: String,
    pub email: String,
    pub location: String,
    pub major: String,
}

#[derive(Serialize, FromRow)] //user table
struct User {
    userid: String, //pk
    password: String,
    location: String,
    username: String,
    major: String,
}

//post functions
#[post("/create_account")] //post user
pub async fn create_account(state: Data<AppState>, body: Json<CreateUser>) -> impl Responder {
    println!(
        "attempting account creation as: {} | {} | {} | {} | {}",
        body.username, body.password, body.email, body.location, body.major
    );
    let id = Uuid::new_v4();
    match sqlx::query_as::<_, User>(
        "INSERT INTO asu_user (userid, password, username, email, location, major)\
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING userid, password, username, email, location, major",
    )
    .bind(id.to_string())
    .bind(body.password.to_string())
    .bind(body.username.to_string())
    .bind(body.email.to_string())
    .bind(body.location.to_string())
    .bind(body.major.to_string())
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => {

            create_classlist_for_user(state.clone(), id.to_string()).await;
            HttpResponse::Ok().json(user)
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

#[derive(Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Serialize, FromRow)]
struct LoginResponse {
    username: String,
    userid: String,
    location: String,
    major: String,
}

#[post("/login")] //post user
pub async fn login(state: Data<AppState>, body: Json<LoginInfo>) -> impl Responder {
    println!("attempting login as: {} | {}", body.username, body.password);
    match sqlx::query_as::<_, LoginResponse>(
        "SELECT username, userid, location, major FROM asu_user WHERE username = $1 AND password = $2"
    )
        .bind(body.username.to_string())
        .bind(body.password.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to login user"),
    }
}

#[derive(FromRow, Deserialize, Serialize)]
pub struct GetClassListId {
    classlistid: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct AddToWishlist {
    userid: String,
    classid: i32,
    term: i32,
}


#[derive(Serialize, FromRow, Debug)]
struct ClassInfo {
    classid: i32,
    title: String,
    units: i32,
    dates: String,
    status: i32,
    days: String,
    starttime: String,
    endtime: String,
    instructor: Vec<String>,
    location: String,
    course: String,
    session: String,
    term: i32,
}

#[get("/search_class")]
pub async fn search_class(
    state: Data<AppState>,
    Query(info): Query<HashMap<String, String>>,
) -> impl Responder {
    println!("{:?}", info);

    //split search query into hashmap
    let iterable_headers: HashMap<String, String> =
        serde_json::from_value(serde_json::to_value(info).unwrap()).unwrap();

    //start building out sql query
    let mut sql_query: String = "SELECT * FROM class WHERE ".to_owned();
    let mut sql_where: String = "".to_owned();

    //concat sql conditionals
    for value in &iterable_headers {
        if value.0 == "instructor" {
            sql_where.push_str("e'\\'");
            sql_where.push_str(value.1);
            sql_where.push_str("\\'' = ANY(instructor)");
            sql_where.push_str(" AND ");
            continue;
        }

        sql_where.push_str(value.0);
        sql_where.push_str(" = '");
        sql_where.push_str(value.1);
        sql_where.push_str("' AND ");
    }

    let _ = sql_where.split_off(sql_where.len() - 5); //trim loose AND
    sql_query.push_str(&sql_where);
    println!("{}", sql_query);

    //query db
    match sqlx::query_as::<_, ClassInfo>(&sql_query)
        .fetch_all(&state.db)
        .await
    {
        Ok(class_search_results) => HttpResponse::Ok().json(class_search_results),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::NotFound().json("No classes found")
        },
    }
}

#[get("/required/{term}/{subject}/{number}")]
pub async fn get_required(
    path: Path<(String, String, String)>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (term, subject, number) = path.into_inner(); //grab search queries

    //build out api url
    let mut url: String = "https://eadvs-cscc-catalog-api.apps.asu.edu/catalog-microservices/api/v1/search/courses?refine=Y".to_owned();
    url.push_str("&term=");
    url.push_str(&*term);
    url.push_str("&subject=");
    url.push_str(&*subject);
    url.push_str("&catalogNbr=");
    url.push_str(&*number);

    //reqwest get w/ necessary auth header
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(AUTHORIZATION, "Bearer null")
        .send()
        .await?
        .text()
        .await?;

    //regex for prereqs section
    let re = Regex::new(r#"ENROLLREQ":"(.*)","ACAD"#).unwrap();
    let capture = re.captures(&resp);

    match capture {
        //check if regex found a match
        Some(x) => Ok(HttpResponse::Ok().json(x.get(1).unwrap().as_str())),
        None => Ok(HttpResponse::NotFound().json("No prereqs found")),
    }
}
