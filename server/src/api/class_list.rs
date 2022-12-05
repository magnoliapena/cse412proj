use crate::AppState;
use actix_web::{
    delete, get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;
use sqlx;
use uuid::Uuid;

use crate::api::common::{AddToWishlist, ClassInfo, GetClassListId};

// async fn get_class_list_id(state: Data<AppState>, userid: String) -> String {
//     let sql_query =
//         sqlx::query_as::<_, GetClassListId>("SELECT ClassListId FROM wishlist WHERE userid = $1;")
//             .bind(userid.to_string());
//
//     match sql_query.fetch_one(&state.db).await {
//         Ok(result) => result.classlistid,
//         Err(_) => "".to_string(),
//     }
//
// }

#[derive(Deserialize)]
pub struct UserId {
    userid: String,
}

#[post("/user/add_to_wishlist")] // adds a class to wishlist
pub async fn add_to_wishlist(state: Data<AppState>, body: Json<AddToWishlist>) -> impl Responder {
    // queries the user for their wishlist related classlist and gets it's id
    let sql_query =
        sqlx::query_as::<_, GetClassListId>("SELECT ClassListId FROM wishlist WHERE userid = $1;")
            .bind(body.userid.to_string());

    match sql_query.fetch_one(&state.db).await {
        Ok(result) => {
            let class_list_id: GetClassListId = result;

            // inserts the class into the wishlist's classlist
            match sqlx::query(
                "INSERT INTO class_list_relationship (ClassListId, ClassId, Term) VALUES ($1, $2, $3);",
            )
            .bind(class_list_id.classlistid.to_string())
            .bind(body.classid)
            .bind(body.term)
            .execute(&state.db)
            .await
            {
                Ok(_) => HttpResponse::Ok().json("ok"),
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError().json("Failed to add to wishlist")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to get classlistid"),
    }
}

#[get("/user/get_wishlist")]
pub async fn get_wishlist(state: Data<AppState>, body: Json<UserId>) -> impl Responder {
    let sql_query =
        sqlx::query_as::<_, GetClassListId>("SELECT ClassListId FROM wishlist WHERE userid = $1;")
            .bind(body.userid.to_string());

    match sql_query.fetch_one(&state.db).await {
        Ok(result) => {
            let class_list_id: GetClassListId = result;
            println!("classlistid: {}", class_list_id.classlistid);
            match sqlx::query_as::<_, ClassInfo>(
                "SELECT * FROM class, class_list_relationship \
                WHERE class_list_relationship.ClassListId = $1 \
                AND class.ClassId=class_list_relationship.ClassId \
                AND class.Term=class_list_relationship.Term;"
            )
            .bind(class_list_id.classlistid.to_string())
            .fetch_all(&state.db).await {
                Ok(wishlist) => {
                    HttpResponse::Ok().json(wishlist)
                },
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError().json("Failed to get wishlist 2")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to add to wishlist"),
    }
}

#[delete("/user/delete_from_wishlist")]
pub async fn delete_from_wishlist(
    state: Data<AppState>,
    body: Json<AddToWishlist>,
) -> impl Responder {
    let sql_query =
        sqlx::query_as::<_, GetClassListId>("SELECT ClassListId FROM wishlist WHERE userid = $1;")
            .bind(body.userid.to_string());

    match sql_query.fetch_one(&state.db).await {
        Ok(result) => {
            let class_list_id: GetClassListId = result;

            // deletes the class into the wishlist's classlist
            match sqlx::query(
                "DELETE FROM class_list_relationship WHERE classlistid = $1 AND classid = $2 AND term = $3;",
            )
            .bind(class_list_id.classlistid.to_string())
            .bind(body.classid)
            .bind(body.term)
            .execute(&state.db)
            .await
            {
                Ok(_) => HttpResponse::Ok().json("ok"),
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError().json("Failed to remove from the list, class or user not found")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to get classlistid"),
    }
}

#[post("/user/add_to_takenlist")] // adds a class to taken list
pub async fn add_to_takenlist(state: Data<AppState>, body: Json<AddToWishlist>) -> impl Responder {
    // queries the user for their takenlist related classlist and gets it's id
    let sql_query =
        sqlx::query_as::<_, GetClassListId>("SELECT ClassListId FROM takenlist WHERE userid = $1;")
            .bind(body.userid.to_string());

    match sql_query.fetch_one(&state.db).await {
        Ok(result) => {
            let class_list_id: GetClassListId = result;

            // inserts the class into the takenlist's classlist
            match sqlx::query(
                "INSERT INTO class_list_relationship (ClassListId, ClassId, Term) VALUES ($1, $2, $3);",
            )
            .bind(class_list_id.classlistid.to_string())
            .bind(body.classid)
            .bind(body.term)
            .execute(&state.db)
            .await
            {
                Ok(_) => HttpResponse::Ok().json("ok"),
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError().json("Failed to add to takenlist")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to get classlistid"),
    }
}

#[get("/user/get_takenlist")]
pub async fn get_takenlist(state: Data<AppState>, body: Json<UserId>) -> impl Responder {
    let sql_query =
        sqlx::query_as::<_, GetClassListId>("SELECT ClassListId FROM takenlist WHERE userid = $1;")
            .bind(body.userid.to_string());

    match sql_query.fetch_one(&state.db).await {
        Ok(result) => {
            let class_list_id: GetClassListId = result;
            println!("classlistid: {}", class_list_id.classlistid);
            match sqlx::query_as::<_, ClassInfo>(
                "SELECT * FROM class, class_list_relationship \
                WHERE class_list_relationship.ClassListId = $1 \
                AND class.ClassId=class_list_relationship.ClassId \
                AND class.Term=class_list_relationship.Term;"
            )
            .bind(class_list_id.classlistid.to_string())
            .fetch_all(&state.db)
            .await {
                Ok(takenlist) => {
                    HttpResponse::Ok().json(takenlist)
                },
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError().json("Failed to get takenlist 2")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to add to takenlist"),
    }
}

#[delete("/user/delete_from_takenlist")]
pub async fn delete_from_takenlist(
    state: Data<AppState>,
    body: Json<AddToWishlist>,
) -> impl Responder {
    let sql_query =
        sqlx::query_as::<_, GetClassListId>("SELECT ClassListId FROM takenlist WHERE userid = $1;")
            .bind(body.userid.to_string());

    match sql_query.fetch_one(&state.db).await {
        Ok(result) => {
            let class_list_id: GetClassListId = result;

            // deletes the class into the takesn's classlist
            match sqlx::query(
                "DELETE FROM class_list_relationship WHERE classlistid = $1 AND classid = $2 AND term = $3;",
            )
            .bind(class_list_id.classlistid.to_string())
            .bind(body.classid)
            .bind(body.term)
            .execute(&state.db)
            .await
            {
                Ok(_) => HttpResponse::Ok().json("ok"),
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError().json("Failed to remove from the taken list, class or user not found")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to get classlistid"),
    }
}


pub async fn create_classlist_for_user(state: Data<AppState>, userid: String) {
    let class_list_id = Uuid::new_v4();
    match sqlx::query("INSERT INTO class_list (ClassListId, Term) VALUES ($1, $2);").bind(class_list_id.to_string()).bind(2214).execute(&state.db).await{
        Ok(_) => println!("created class_list"),
        Err(e) => println!("could not create class list when creating user account because of {} with userid {}", e, userid.to_string())
    }

    match sqlx::query("INSERT INTO wishlist (UserId, ClassListId) VALUES ($1, $2);").bind(userid.clone()).bind(class_list_id.to_string()).execute(&state.db).await{
        Ok(_) => println!("created wishlist for user {}", userid.clone()),
        Err(e) => println!("could not create class list when creating user account because of {} with userid {}", e, userid.to_string())
    }

    match sqlx::query("INSERT INTO takenlist (UserId, ClassListId) VALUES ($1, $2);").bind(userid.clone()).bind(class_list_id.to_string()).execute(&state.db).await{
        Ok(_) => println!("created takenlist for user {}", userid.clone()),
        Err(e) => println!("could not create class list when creating user account because of {} with userid {}", e, userid.to_string())
    }
}
