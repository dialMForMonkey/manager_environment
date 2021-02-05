
mod application;
mod actor_models;
mod infrastructure;


use actix_web;
use actix_web::{HttpServer, App, web, HttpResponse};
use actix_web::middleware::Logger;
use env_logger::{Env};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(||
        App::new()
            .wrap(Logger::default())
            .service(application::manager_environment_routers::insert_environment)
            .service(application::manager_environment_routers::list_environment)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
        )
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
