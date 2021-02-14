use actix_web::{http::StatusCode, post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::err::ResponseError;

#[post("/build/plugin/{name}")]
pub async fn build_plugin(
    name: web::Path<String>,
    plugin_content: web::Json<PluginContent>,
) -> impl Responder {
    web::Json(name);
    web::Json(plugin_content);

    web::Json(ResponseError::new(500, String::from("recovery")))
        .with_status(StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize, Serialize)]
pub struct PluginContent {
    pub id: usize,
    pub path: String,
    pub content: String,
}
