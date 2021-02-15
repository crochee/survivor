use std::fs;

use actix_web::{http::StatusCode, post, web, HttpResponse, Responder};
use log::error;

use crate::ServiceError;
use crate::SAVE_ROOT_PATH;

#[post("/bucket/{bucket_name}")]
pub async fn create_bucket(web::Path(bucket_name): web::Path<String>) -> impl Responder {
    let path: String = format!("{}/{}", SAVE_ROOT_PATH, bucket_name);
    match fs::create_dir_all(path) {
        Err(e) => {
            error!("{}", e);
            return HttpResponse::Ok()
                .json(ServiceError::new(500, e.to_string()))
                .with_status(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(()) => {
            return HttpResponse::Ok()
                .json("test")
                .with_status(StatusCode::NO_CONTENT);
        }
    };
}
