use chrono::prelude::*;
use futures_util::{SinkExt, StreamExt};
use kraken::{
    core::CurrencyPair,
    message::{publication::Trade, request::subscribe},
};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    /*let res = reqwest::get("https://api.kraken.com/0/private/GetWebSocketsToken")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    */

    //println!("{:#?}", res);

    let resp = reqwest::get("https://api.kraken.com/0/public/AssetPairs")
        .await
        .unwrap();
    let info = serde_json::from_str::<kraken::rest_api::response::AssetPairs>(
        resp.text().await.unwrap().as_str(),
    )
    .unwrap();
    println!("{:#?}", info);

    return;

    let (mut stream, _) = tokio_tungstenite::connect_async("wss://ws.kraken.com")
        .await
        .unwrap();

    let pair = "ETH/USD".parse::<CurrencyPair>().unwrap();
    let subscribe = subscribe::Subscribe::builder()
        .pair(vec![pair])
        .subscription(subscribe::Subscription::builder().name("trade").build())
        .build();

    stream
        .send(Message::text(serde_json::to_string(&subscribe).unwrap()))
        .await
        .unwrap();

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(Message::Close(_)) => {
                println!("Closing...");
                break;
            }
            Ok(Message::Text(msg)) => match serde_json::from_str::<Trade>(&msg) {
                Ok(trade) => println!("{:#?}", trade),
                Err(_) => {
                    println!("other message: {}", msg);
                }
            },
            Err(e) => {
                eprintln!("{:#?}", e);
                panic!(e);
            }
            Ok(other) => {
                println!("unknown...");
                println!("{:?}", other);
            }
        }
    }
}
