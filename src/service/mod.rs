pub mod binance_strategy;
mod common;
pub mod diff_rate;
pub mod price;
pub mod stable_coin_hedging;
mod diff_price;

pub use diff_price::set_price_data;
pub use diff_price::get_diff_signal;
pub use binance_strategy::event_start;
pub use binance_strategy::inspect_strategy;
pub use binance_strategy::range_new_strategy;
pub use diff_rate::set_binance_diff_rate;
pub use price::get_binance_price;
pub use price::set_binance_price;
pub use stable_coin_hedging::event_stable_coin_start;
pub use stable_coin_hedging::inspect_stable_coin;

use crate::binance::websockets::*;
use crate::binance::ws_model::*;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use crate::constants::{BYBIT_MESSAGE, KUCOIN_MESSAGE};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceStream {
    pub tickers: Vec<BinanceTickerEvent>,
    pub market: Market,
    pub platform: Platform,
    pub local_time: i64,
}



pub async fn handle_websocket(
    url: &str,
    message: Option<&str>,
    platform: Platform,
    market: Market,
    price_tx: UnboundedSender<PriceStream>,
    keep_running: &AtomicBool,
) {
    loop {
        let mut web_socket: WebSockets<'_, WebSocketEvent> = WebSockets::new(|event: WebSocketEvent| {
            let mut tickers = Vec::new();
            match platform {
                Platform::Binance => {
                    if let WebSocketEvent::MiniTicker(tick_events) = event {
                        tickers.extend(*tick_events);
                    }
                }
                Platform::Bybit => {
                    if let WebSocketEvent::TickerDelta(bybit_event) = event {
                        tickers.push(bybit_event.into());
                    }
                }
                Platform::Kucoin => {
                    if let WebSocketEvent::KucoinTicker(ckucoin_event) = event {
                        tickers.push(ckucoin_event.into());
                    }
                }
            }

            let price_stream = PriceStream {
                tickers,
                platform: platform.clone(),
                market: market.clone(),
                local_time: chrono::Local::now().timestamp_millis(),
            };

            if price_tx.send(price_stream).is_err() {
                keep_running.store(false, Ordering::Relaxed);
            }

            Ok(())
        });

        web_socket.connect(Url::parse(url).unwrap()).await.unwrap();
        if let Some(msg) = message {
            web_socket.send_message(Message::Text(msg.to_string())).await.unwrap();
        }
        if let Err(e) = web_socket.event_loop(&keep_running).await {
            error!("Error: {e}");
            continue
        }
        web_socket.disconnect().await.unwrap();
        info!("websocket disconnected");
    }

}

pub async fn binance_spot_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let url = "wss://stream.binance.com:9443/ws/!miniTicker@arr";

    handle_websocket(url, None, Platform::Binance, Market::Spot, price_tx, &keep_running).await;
}

pub async fn binance_futures_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let url = "wss://fstream.binance.com/ws/!miniTicker@arr";

    handle_websocket(url, None, Platform::Binance, Market::Futures, price_tx, &keep_running).await;
}

pub async fn bybit_spot_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let message = BYBIT_MESSAGE;
    let url = "wss://stream.bybit.com/contract/usdt/public/v3";

    handle_websocket(url, Some(message), Platform::Bybit, Market::Spot, price_tx, &keep_running).await;
}

pub async fn bybit_futures_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let message = BYBIT_MESSAGE;
    let url = "wss://stream.bybit.com/contract/usdt/public/v3";

    handle_websocket(url, Some(message), Platform::Bybit, Market::Futures, price_tx, &keep_running).await;
}

pub async fn kucoin_spot_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let message = KUCOIN_MESSAGE;
    let url = "wss://push1-v2.kucoin.com/endpoint";

    handle_websocket(url, Some(message), Platform::Kucoin, Market::Spot, price_tx, &keep_running).await;
}

pub async fn kucoin_futures_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    let keep_running = AtomicBool::new(true);
    let message = KUCOIN_MESSAGE;
    let url = "wss://ws-api.kucoin.com/endpoint?token=2neAiuYvAU61ZDXANAGAsiL4-iAExhsBXZxftpOeh_55i3Ysy2q2LEsEWU64mdzUOPusi34M_wGoSf7iNyEWJ_K5H63P8kFWJc4EK10_nKiOZxmSM3PY-9iYB9J6i9GjsxUuhPw3BlrzazF6ghq4LzJLZP8BvaT_BRUoz0xJxR0=.G2UFzARHJwRctOldHX6dyw==";

    handle_websocket(url, Some(message), Platform::Kucoin, Market::Futures, price_tx, &keep_running).await;
}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn binance_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    tokio::join!(
        // binance_spot_all_ticker(price_tx.clone()),
        binance_futures_all_ticker(price_tx.clone()),
    );
}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn bybit_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    tokio::join!(
        // bybit_spot_all_ticker(price_tx.clone()),
        bybit_futures_all_ticker(price_tx.clone()),
    );
}

#[allow(dead_code)]
#[allow(irrefutable_let_patterns)]
pub async fn kucoin_all_ticker(price_tx: UnboundedSender<PriceStream>) {
    tokio::join!(
        // kucoin_spot_all_ticker(price_tx.clone()),
        kucoin_futures_all_ticker(price_tx.clone())
    );
}
//
// #[allow(dead_code)]
// #[allow(irrefutable_let_patterns)]
// pub async fn delivery_all_ticker(price_tx: UnboundedSender<PriceStream>) {
//     let keep_running = AtomicBool::new(true);
//     let all_ticker = all_mini_ticker_stream();
//
//     let mut web_socket: WebSockets<'_, Vec<WebsocketEvent>> =
//         WebSockets::new(|events: Vec<WebsocketEvent>| {
//             let mut tickers = Vec::new();
//             for tick_events in events {
//                 if let WebsocketEvent::DayMiniTicker(tick_event) = tick_events {
//                     tickers.push(*tick_event)
//                 }
//             }
//
//             let price_stream = PriceStream {
//                 tickers: tickers,
//                 market: "delivery".to_string(),
//                 local_time: chrono::Local::now().timestamp_millis(),
//             };
//             match price_tx.send(price_stream) {
//                 Ok(_) => {}
//                 Err(_e) => {
//                     keep_running.store(false, Ordering::Relaxed);
//                 }
//             }
//
//             // Break the event loop
//             // keep_running.store(false, Ordering::Relaxed);
//             Ok(())
//         });
//
//     web_socket.connect_delivery(all_ticker).await.unwrap(); // check error
//     if let Err(e) = web_socket.event_loop(&keep_running).await {
//         error!("Error: {e}");
//     }
//     web_socket.disconnect().await.unwrap();
//     info!("delivery websocket disconnected");
// }
