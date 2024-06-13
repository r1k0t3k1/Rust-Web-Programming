mod create;
mod get;
mod edit;

use actix_web::web::{ ServiceConfig, get, post, scope };

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    let factory = scope("v1/item")
                            .route("create/{title}", get().to(create::create))
                            .route("get", get().to(get::get))
                            .route("edit", post().to(edit::edit));
    app.service(factory);
}
