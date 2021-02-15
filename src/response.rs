use std::{error::Error, fmt};

use serde::Serialize;

/// web uses ServiceError to response some error
/// # Example
/// ```rust
/// use actix_web::{http::StatusCode, post, web, HttpResponse, Responder};
///
/// use crate::ServiceError;
///
/// #[post("/bucket/{bucket_name}")]
/// pub async fn create_bucket(web::Path(bucket_name): web::Path<String>) -> impl Responder {
///               HttpResponse::Ok()
///                     .json(ServiceError::new(500, bucket_name))
///                     .with_status(StatusCode::INTERNAL_SERVER_ERROR)
/// }
/// ```
///
#[derive(Debug, Serialize)]
pub struct ServiceError {
    message: String,
    code: usize,
}

impl ServiceError {
    pub fn new(code: usize, message: String) -> Self {
        ServiceError { code, message }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Internal Server response Error!code:{},msg:{}",
            self.code, self.message
        )
    }
}

impl Error for ServiceError {}

#[test]
fn test_response_error() {
    let err = ServiceError::new(500, String::from("recovery"));
    println!("{}", err)
}
