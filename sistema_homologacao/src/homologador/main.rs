use actix_web::{
    web, App, HttpServer, 
    middleware::Logger
};
use std::sync::Mutex;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Estado compartilhado para armazenar resultados de homologação
    let homologation_data = web::Data::new(Mutex::new(models::HomologationData::new()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(homologation_data.clone())
            .service(handlers::index)
            .service(handlers::submit_homologation)
            .service(handlers::get_results)
            .service(handlers::get_report)
            .service(actix_files::Files::new("/static", "static"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}