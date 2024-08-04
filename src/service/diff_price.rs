use crate::{model, service, sql};
use chrono::{Duration, Local, TimeDelta, Utc};
use log::{debug, error};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::ops::{Div, Sub};
use std::str::FromStr;
use tokio::select;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::service::PriceStream;

pub async fn set_price_data(mut price_rx: UnboundedReceiver<PriceStream>) {
    let up = (Utc::now() + Duration::hours(2)).naive_utc().to_string();
    loop {
        select! {
            event = price_rx.recv() => {
                if let Some(stream) = event {
                    for ticker in stream.tickers {
                        let symbol = ticker.symbol.clone();
                        let price = Decimal::from_str(&ticker.current_close).unwrap();
                        let market = stream.market.clone();
                        let updated = (Utc::now() + Duration::hours(2)).naive_utc().to_string();


                        // Check if the record exists
                        if let Ok(existing_record) = sql::get_arb_coin_price_by_symbol(&symbol).await {
                            let (spot_price, future_price) = if market == "spot" {
                                (price, existing_record.future_price.clone())
                            } else if market == "futures" {
                                (existing_record.spot_price.clone(), price)
                            } else {
                                (existing_record.spot_price.clone(), existing_record.future_price.clone()) // Default case, should not happen
                            };
                            // Update the existing record
                            if let Err(e) = sql::update_arb_coin_price_by_id(
                                existing_record.id,
                                spot_price,
                                future_price,
                                updated.clone(),
                            ).await {
                                error!("{:?}", e);
                            }
                        } else {
                            let (spot_price, future_price) = if market == "spot" {
                                (price, Decimal::ZERO)
                            } else if market == "future" {
                                (Decimal::ZERO, price)
                            } else {
                                (Decimal::ZERO, Decimal::ZERO) // Default case, should not happen
                            };
                            // Insert a new record
                            if let Err(e) = sql::insert_arb_coin_price(
                                model::ArbCoinPrice {
                                    id: 0,
                                    platform: "binance".to_string(),
                                    symbol,
                                    spot_price,
                                    future_price,
                                    created: updated.clone(),
                                    updated,
                                    bak: None
                                }
                            ).await {
                                error!("{:?}", e);
                            }
                        }
                    }
                }
            },
        }
    }
}

#[allow(unused_assignments)]
#[allow(unused_assignments)]
pub async fn get_diff_signal() {
    loop {

        // Fetch all rows from the arb_coin_price table
        let coin_prices = sql::get_all_arb_coin_prices().await.unwrap();

        for coin_price in coin_prices {
            // Calculate the difference between future_price and spot_price
            if coin_price.future_price == Decimal::ZERO || coin_price.spot_price == Decimal::ZERO {
                continue;
            }
            let price_diff = coin_price.future_price.sub(coin_price.spot_price);
            let price_diff_rate = price_diff.div(coin_price.future_price);

            // Set the threshold to 0.3
            let threshold = Decimal::from_str("0.003").unwrap();
            let updated = (Utc::now() + Duration::hours(2)).naive_utc().to_string();

            // Check if the difference is greater than the threshold
            if price_diff_rate > threshold {
                // Check if the entry already exists in the diff_signal table
                if let Ok(existing_signal) = sql::get_arb_diff_signal_by_symbol(&coin_price.symbol).await {
                    // Update the existing record
                    if let Err(e) = sql::update_arb_diff_signal_by_id(
                        existing_signal.id,
                        price_diff,
                        price_diff_rate,
                        coin_price.spot_price.clone(),
                        coin_price.future_price.clone(),
                        updated.clone(),
                    ).await {
                        error!("{:?}", e);
                    }
                } else {
                    // Insert a new record
                    if let Err(e) = sql::insert_arb_diff_signal(
                        model::ArbDiffSignal {
                            id: 0,
                            symbol: coin_price.symbol.clone(),
                            arb_coin_price_id: coin_price.id,
                            price_diff,
                            price_diff_rate,
                            spot_price: coin_price.spot_price.clone(),
                            future_price: coin_price.future_price.clone(),
                            updated: updated.clone(),
                            created: updated.clone(),
                            bak: None,
                        }
                    ).await {
                        error!("{:?}", e);
                    }
                }
            }
        }
    }
}