use crate::model::class;

#[allow(unused_imports)]
use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    web::Query,
    HttpResponse,
};

#[get("/api/class")]
pub async fn get_class(class_identifier: Query<class::ClassIdentifier>) -> Json<String> {
    return Json(class_identifier.into_inner().class_id);
}
