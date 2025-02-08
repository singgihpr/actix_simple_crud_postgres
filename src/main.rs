use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod handlers;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(handlers::create_user)
            .service(handlers::get_users)
            .service(handlers::get_user_by_id)
            .service(handlers::update_user)
            .service(handlers::delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}