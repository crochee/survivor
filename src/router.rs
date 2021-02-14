use actix_web::web::{scope, ServiceConfig};

use crate::controller::build;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").service(build::build_plugin));
}
