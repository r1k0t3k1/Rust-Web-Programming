mod views;
mod to_do;
mod processes;
mod state;
mod json_serialization;
mod jwt;

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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
