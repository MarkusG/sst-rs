// use time::OffsetDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "transaction_id"))]
    id: String,
    account_id: String,
    amount: f64,
    // date: OffsetDateTime,
    // datetime: OffsetDateTime,
    merchant_name: Option<String>,
    #[serde(rename(deserialize = "name"))]
    description: String
}
