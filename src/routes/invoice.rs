use crate::controllers::invoice_controller;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/invoice/list")
            .route(web::get().to(invoice_controller::list_invoices)) // GET route
            .route(web::post().to(invoice_controller::list_invoices)), // POST route
    );
}
