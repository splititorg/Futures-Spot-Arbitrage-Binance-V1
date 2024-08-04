use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, sqlx::FromRow)]
pub struct ArbDiffSignal {
    pub id: i64,
    pub symbol: String,
    pub arb_coin_price_id: i64,
    pub price_diff: Decimal,
    pub price_diff_rate: Decimal,
    pub created: String,
    pub updated: String,
    pub bak: Option<String>,
}
