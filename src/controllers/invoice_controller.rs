use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use crate::utils::response::response::{create_response};
use crate::utils::response::response_code::ResponseCode;
use crate::models::invoice::Invoice;
use crate::models::invoice_response::InvoiceResponse;


pub async fn list_invoices(db_pool: web::Data<MySqlPool>) -> HttpResponse {
    match Invoice::get_all(&db_pool).await {
        Ok(invoices) => {
            // Convert `Invoice` instances to `InvoiceResponse`
            let invoices: Vec<InvoiceResponse> = invoices
                .into_iter()
                .map(InvoiceResponse::from) // Use the From trait for conversion
                .collect();

            create_response(
                Some("success".to_string()),
                Some("Invoices fetched successfully".to_string()),
                ResponseCode::SUCCESS,
                Some(invoices),
                None,
                None,
                None,
            )
        }
        Err(err) => create_response(
            Some("error".to_string()),
            Some("Failed to fetch invoices".to_string()),
            ResponseCode::DBQUERYERROR,
            None::<Vec<InvoiceResponse>>, // Provide appropriate type for None
            None,
            Some(serde_json::json!({"error_details": format!("{:?}", err)})),
            None,
        ),
    }
}
