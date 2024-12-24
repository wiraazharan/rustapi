use crate::controllers::test_controller;
use actix_web::web;

// pub fn configure_routes(cfg: &mut web::ServiceConfig) {
//     cfg.route("/test", web::get().to(test_controller::hello_world_handler));
// }

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(test_controller::hello_world_handler))
            .route(web::post().to(test_controller::hello_world_handler)),
    );
}
