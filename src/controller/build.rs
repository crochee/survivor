use actix_web::{post, web, Responder};
use log::info;
use serde::{Deserialize, Serialize};

#[post("/build/plugin/{name}")]
pub async fn build_plugin(
    web::Path(name): web::Path<String>,
    plugin_content: web::Json<PluginContent>,
) -> impl Responder {
    info!("{:#?}", plugin_content);
    name
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginContent {
    pub id: usize,
    pub path: String,
    pub content: String,
}
