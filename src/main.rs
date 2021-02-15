// 导入包 分私有和公共 公共可以供别的包或者作为第三方调用 私有只能在当前包使用
// 一般项目都是在main.rs或者lib.rs的main函数下，进行包导入，这样的可以让调用方从包的树结构从根(crate)开始调用
mod config;
pub mod controller;
mod logger;
mod response;
pub mod router;

// use一般规则 标准库-》三方库-》自定义的顺序分类
// 带pub 的可供调用方感知，一般用法为暴露给调用方访问
use actix_web::{middleware, App, HttpServer};
use log::info;

pub use crate::config::SAVE_ROOT_PATH;
pub use crate::response::ServiceError;

/// rust web use actix-web 3.3.2 demo,
/// it's uesd to study rust
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