use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub const DIFF_STATUS_UN_RUN: i8 = 0;
pub const DIFF_STATUS_RUN: i8 = 1;

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbCoinPrice {
    pub id: i64,
    pub platform: String,
    pub symbol: String,
    pub spot_price: Decimal,
    pub future_price: Decimal,
    pub created: String,
    pub updated: String,
    pub bak: Option<String>,
}

