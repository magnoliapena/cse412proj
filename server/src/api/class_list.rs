use actix_identity::Identity;
use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::utils::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    subject: String,
    teacher: String,
    location: String,
}

impl Class {
    pub fn all() -> Vec<Class> {
        vec![
            Class {
                subject: "CSE69".into(),
                teacher: "Me".into(),
                location: "your mom".into(),
            },
            Class {
                subject: "CSE420".into(),
                teacher: "Myself".into(),
                location: "nowhere".into(),
            },
            Class {
                subject: "CSE69420".into(),
                teacher: "I".into(),
                location: "everywhere".into(),
            },
        ]
    }
}

pub async fn get_all(user: Option<Identity>) -> Result<web::Json<Vec<Class>>, Error> {
    println!("[class_list] - get all classes");

    if let Some(user) = user {
        println!("[class_list] - username: {}", user.id().unwrap());

        Ok(web::Json(Class::all()))
    } else {
        println!("[class_list] - username: failed");

        Err(Error {
            message: "Auth failed".into(),
            status: 401,
        })
    }
}
