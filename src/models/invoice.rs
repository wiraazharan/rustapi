use chrono::NaiveDateTime; // Or use `sqlx::types::chrono::NaiveDateTime` if using sqlx.
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{types::BigDecimal, FromRow};
use sqlx::{MySqlPool, Result};

#[derive(Debug, FromRow, Serialize)]
pub struct Invoice {
    pub id: u64,
    pub invoice_no: String,
    pub amount: BigDecimal,
    pub charge_rate: BigDecimal,
    pub client_access_token_id: Option<i32>, // Match SQL INT type
    pub payment_service_id: Option<i64>,
    pub switch_integration_provider_code: Option<String>,
    pub invoice_status_id: Option<i32>,
    pub client_redirect_url: Option<String>,
    pub client_callback_url: Option<String>,
    pub client_transaction_details: Option<String>,
    pub created_at: Option<DateTime<Utc>>, // Matches SQL TIMESTAMP
    pub updated_at: Option<DateTime<Utc>>, // Matches SQL TIMESTAMP
    pub no_of_confirmation_attempts: i32,
    pub payment_switch_charge_rate: BigDecimal,
    pub payment_switch_charge_rate_details: Option<String>,
    pub client_data: Option<String>,
    pub client_imposed_fee: BigDecimal,
    pub client_imposed_fee_type_id: i32,
    pub client_imposed_fee_applicable_id: i32,
    pub client_profit: BigDecimal,
    pub invoice_type_id: i32,
    pub payment_switch_charge_rate_after_reconcile: Option<BigDecimal>,
    pub reconcile_done: i32,
    pub reconcile_channel: Option<String>,
    pub reconcile_details: Option<String>,
    pub reconcile_provider_rate: Option<BigDecimal>,
    pub reconcile_provider_rate_type_reference_id: Option<i32>,
    pub client_invoice_no: Option<String>,
    pub provider_transaction_reference: Option<String>,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
}

impl Invoice {
    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Invoice>> {
        let invoices = sqlx::query_as::<_, Invoice>("SELECT * FROM invoices")
            .fetch_all(pool)
            .await?;
        Ok(invoices)
    }
}
