use log::info;

use actix_web::{middleware, App, HttpServer};

mod logger;

pub mod controller;
pub mod err;
pub mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();
    info!("run ...");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(router::router)
    })
    .bind(":8083")
    .expect("bind addr failed")
    .run()
    .await
}
