#[macro_use]
extern crate diesel;
extern crate dotenv;

mod views;
mod to_do;
mod json_serialization;
mod jwt;
mod config;

mod database;
mod models;
mod schema;

use actix_web::{ App, HttpServer };
use actix_service::Service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    Ok(future.await?)
                }
            })
            .configure(views::views_factory);
        return app
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
