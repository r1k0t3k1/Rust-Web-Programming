mod create;

use actix_web::web::{ ServiceConfig, get, scope };

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    let factory = scope("v1/item")
                            .route("create/{title}", get().to(create::create));
    app.service(factory);
}
