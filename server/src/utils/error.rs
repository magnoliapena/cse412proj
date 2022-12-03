use std::fmt::{ Display, Formatter, Result as FmtResult };
use actix_web::{ ResponseError, HttpResponse };
use actix_web::http::StatusCode;
use serde::Serialize;
use serde_json::{ json, to_string_pretty };

#[derive(Debug, Serialize)]
pub struct Error {
    pub message: String,
    pub status: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.message });
        HttpResponse::build(
            StatusCode::from_u16(self.status).unwrap()
        ).json(err_json)
    }
}