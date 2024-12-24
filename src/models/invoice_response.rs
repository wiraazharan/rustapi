use chrono::{NaiveDateTime, DateTime, Utc};
use serde::Serialize;
use sqlx::types::BigDecimal;
use crate::models::invoice::Invoice;

#[derive(Serialize)]
pub struct InvoiceResponse {
    pub id: u64,
    pub invoice_no: String,
    pub amount: BigDecimal,
    pub charge_rate: BigDecimal,
    pub client_access_token_id: Option<u64>,
    pub created_at: Option<NaiveDateTime>, // Use NaiveDateTime for API response
    pub updated_at: Option<NaiveDateTime>, // Use NaiveDateTime for API response
}

impl From<Invoice> for InvoiceResponse {
    fn from(invoice: Invoice) -> Self {
        Self {
            id: invoice.id,
            invoice_no: invoice.invoice_no,
            amount: invoice.amount,
            charge_rate: invoice.charge_rate,
            client_access_token_id: invoice.client_access_token_id.map(|id| id as u64),
            created_at: invoice.created_at.map(|dt| dt.naive_utc()), // Explicit conversion to NaiveDateTime
            updated_at: invoice.updated_at.map(|dt| dt.naive_utc()), // Explicit conversion to NaiveDateTime
        }
    }
}
