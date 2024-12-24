use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub id: i32,
    pub invoice_no: String,
    pub amount: f64,
}
