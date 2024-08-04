use crate::{db, model};
use chrono::Local;
use rust_decimal::Decimal;

pub async fn get_all_arb_coin_prices() -> anyhow::Result<Vec<model::ArbCoinPrice>> {
    let diff_rate_list = sqlx::query_as::<_, model::ArbCoinPrice>(
        "select * from arb_coin_price",
    )
    .fetch_all(db::get_db()?.database())
    .await?;
    Ok(diff_rate_list)
}

pub async fn get_arb_coin_price_by_symbol(
    symbol: &String,
) -> anyhow::Result<model::ArbCoinPrice> {
    let coin_price_list = sqlx::query_as::<_, model::ArbCoinPrice>(
        "select * from arb_coin_price where symbol = ?",
    ).bind(symbol)
        .fetch_one(db::get_db()?.database())
        .await?;
    Ok(coin_price_list)
}

pub async fn update_arb_coin_price_by_id(
    id: i64,
    spot_price: Decimal,
    future_price: Decimal,
    updated: String,
) -> anyhow::Result<u64> {
    let rows = sqlx::query("update arb_coin_price set spot_price = ?, future_price = ?, updated = ? where id = ?")
        .bind(spot_price)
        .bind(future_price)
        .bind(updated)
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}

pub async fn insert_arb_coin_price(coin_price: model::ArbCoinPrice) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query("insert into arb_coin_price (platform, symbol, spot_price, future_price, created, updated) values (?, ?, ?, ?, ?, ?)")
        .bind(coin_price.platform)
        .bind(coin_price.symbol)
        .bind(coin_price.spot_price)
        .bind(coin_price.future_price)
        .bind(coin_price.created)
        .bind(coin_price.updated)
        .execute(db::get_db()?.database())
        .await?
        .last_insert_id();
    Ok(last_insert_id)
}

pub async fn insert_arb_diff_signal(diff_signal: model::ArbDiffSignal) -> anyhow::Result<u64> {
    let last_insert_id = sqlx::query(
        "insert into arb_diff_signal (
        symbol,
        arb_coin_price_id,
        price_diff,
        price_diff_rate,
        spot_price,
        future_price,
        created,
        updated
        ) values (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(diff_signal.symbol)
    .bind(diff_signal.arb_coin_price_id)
    .bind(diff_signal.price_diff)
    .bind(diff_signal.price_diff_rate)
    .bind(diff_signal.spot_price)
    .bind(diff_signal.future_price)
    .bind(diff_signal.created)
    .bind(diff_signal.updated)
    .execute(db::get_db()?.database())
    .await?
    .last_insert_id();
    Ok(last_insert_id)
}

pub async fn get_arb_diff_signal_by_symbol(symbol: &String) -> anyhow::Result<model::ArbDiffSignal> {
    let diff_signal = sqlx::query_as::<_, model::ArbDiffSignal>(
        "SELECT * FROM arb_diff_signal WHERE symbol = ?",
    )
        .bind(symbol)
        .fetch_one(db::get_db()?.database())
        .await?;
    Ok(diff_signal)
}

pub async fn update_arb_diff_signal_by_id(
    id: i64,
    price_diff: Decimal,
    price_diff_rate: Decimal,
    spot_price: Decimal,
    future_price: Decimal,
    updated: String,
) -> anyhow::Result<u64> {
    let rows = sqlx::query("update arb_diff_signal set price_diff = ?, price_diff_rate = ?, spot_price = ?, future_price = ?, updated = ? where id = ?")
        .bind(price_diff)
        .bind(price_diff_rate)
        .bind(spot_price)
        .bind(future_price)
        .bind(updated)
        .bind(id)
        .execute(db::get_db()?.database())
        .await?
        .rows_affected();
    Ok(rows)
}
