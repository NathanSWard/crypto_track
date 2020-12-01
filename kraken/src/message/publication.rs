pub mod system_status {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum SystemStatusEvent {
        SystemStatus,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
    #[serde(rename_all = "lowercase")]
    pub enum Status {
        Online,
        Maintenance,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct SystemStatus {
        event: SystemStatusEvent,
        #[serde(rename = "connectionID")]
        pub connection_id: u64,
        pub status: Status,
        pub version: String,
    }
}

pub mod hearbeat {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum HeartbeatEvent {
        Heartbeat,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Heartbeat {
        event: HeartbeatEvent,
    }
}

pub mod ticker {
    use crate::core::{CurrencyPair, KrakenFloat};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Ask {
        pub price: KrakenFloat,
        pub whole_lot_volume: i64,
        pub lot_volume: KrakenFloat,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Bid {
        pub price: KrakenFloat,
        pub whole_lot_volume: i64,
        pub lot_volume: KrakenFloat,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Close {
        pub price: KrakenFloat,
        pub lot_volume: KrakenFloat,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct PriceHistory<T> {
        pub today: T,
        #[serde(rename = "last24Hours")]
        pub last_24_hours: T,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Data {
        #[serde(rename = "a")]
        pub ask: Ask,
        #[serde(rename = "b")]
        pub bid: Bid,
        #[serde(rename = "c")]
        pub close: Close,
        #[serde(rename = "v")]
        pub volume: PriceHistory<KrakenFloat>,
        #[serde(rename = "p")]
        pub volumne_weighted_average_price: PriceHistory<KrakenFloat>,
        #[serde(rename = "t")]
        pub number_of_trades: PriceHistory<i64>,
        #[serde(rename = "l")]
        pub low_price: PriceHistory<KrakenFloat>,
        #[serde(rename = "h")]
        pub high_price: PriceHistory<KrakenFloat>,
        #[serde(rename = "o")]
        pub open_price: PriceHistory<KrakenFloat>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Ticker {
        #[serde(rename = "channelID")]
        pub channel_id: i64,
        pub data: Data,
        #[serde(rename = "channelName")]
        pub channel_name: String,
        pub pair: CurrencyPair,
    }
} // mod ticker

pub mod ohlc {
    use crate::core::{CurrencyPair, KrakenFloat};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct Data {
        pub time: KrakenFloat,
        pub etime: KrakenFloat,
        pub open: KrakenFloat,
        pub high: KrakenFloat,
        pub low: KrakenFloat,
        pub close: KrakenFloat,
        pub vwap: KrakenFloat,
        pub volume: KrakenFloat,
        pub count: i64,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Ohlc {
        #[serde(rename = "channelID")]
        pub channel_id: i64,
        pub data: Data,
        #[serde(rename = "channelName")]
        pub channel_name: String,
        pub pair: CurrencyPair,
    }
} // mod ohlc

pub mod trade {
    use crate::core::{CurrencyPair, KrakenFloat};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    #[serde(try_from = "&str")]
    pub enum OrderSide {
        Buy,
        Sell,
    }

    impl std::convert::TryFrom<&str> for OrderSide {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value == "s" {
                Ok(OrderSide::Sell)
            } else if value == "b" {
                Ok(OrderSide::Buy)
            } else {
                Err("invalid trade::OrderSide")
            }
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(try_from = "&str")]
    pub enum OrderType {
        Market,
        Limit,
    }

    impl std::convert::TryFrom<&str> for OrderType {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value == "m" {
                Ok(OrderType::Market)
            } else if value == "l" {
                Ok(OrderType::Limit)
            } else {
                Err("invalid trade::OrderSide")
            }
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Data<TimeRepr> {
        pub price: KrakenFloat,
        pub volume: KrakenFloat,
        pub time: TimeRepr,
        pub side: OrderSide,
        #[serde(rename = "orderType")]
        pub order_type: OrderType,
        pub misc: String,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Trade {
        #[serde(rename = "channelID")]
        pub channel_id: i64,
        pub data: Vec<Data<KrakenFloat>>,
        #[serde(rename = "channelName")]
        pub channel_name: String,
        pub pair: CurrencyPair,
    }
} // mod trade

pub mod spread {
    use crate::core::{CurrencyPair, KrakenFloat};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Data {
        pub bid: KrakenFloat,
        pub ask: KrakenFloat,
        pub timestamp: KrakenFloat,
        pub bid_volume: KrakenFloat,
        pub ask_volume: KrakenFloat,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Spread {
        #[serde(rename = "channelID")]
        pub channel_id: i64,
        pub data: Data,
        #[serde(rename = "channelName")]
        pub channel_name: String,
        pub pair: CurrencyPair,
    }
} // mod spread

pub mod book {
    use crate::core::{CurrencyPair, KrakenFloat};
    use serde::Deserialize;

    pub mod snapshot {
        use super::*;

        #[derive(Debug, Deserialize, Clone)]
        pub struct PriceLevelData {
            pub price: KrakenFloat,
            pub volume: KrakenFloat,
            pub timestamp: KrakenFloat,
        }

        #[derive(Debug, Deserialize, Clone)]
        pub struct Data {
            #[serde(rename = "as")]
            pub ask: Vec<PriceLevelData>,

            #[serde(rename = "bs")]
            pub bid: Vec<PriceLevelData>,
        }

        #[derive(Debug, Deserialize, Clone)]
        pub struct BookSnapshot {
            #[serde(rename = "channelID")]
            pub channel_id: i64,
            data: Data,
            #[serde(rename = "channelName")]
            pub channel_name: String,
            pub pair: CurrencyPair,
        }
    }

    pub mod update {
        // TODO: currently this is very hacky...
        use super::*;

        #[derive(Debug, Deserialize, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct PriceLevelData {
            pub price: KrakenFloat,
            pub volume: KrakenFloat,
            pub timestamp: KrakenFloat,

            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub update_type: Option<String>,
        }

        #[derive(Debug, Deserialize, Clone)]
        pub struct AskUpdate {
            #[serde(rename = "a")]
            updates: Vec<PriceLevelData>,

            #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
            checksum: Option<String>,
        }

        #[derive(Debug, Deserialize, Clone)]
        pub struct BidUpdate {
            #[serde(rename = "b")]
            updates: Vec<PriceLevelData>,

            #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
            checksum: Option<String>,
        }

        #[derive(Debug, Deserialize, Clone)]
        struct Ask {
            channel_id: i64,
            ask: AskUpdate,
            channel_name: String,
            pair: CurrencyPair,
        }

        #[derive(Debug, Deserialize, Clone)]
        struct Bid {
            channel_id: i64,
            bid: BidUpdate,
            channel_name: String,
            pair: CurrencyPair,
        }

        #[derive(Debug, Deserialize, Clone)]
        struct AskBid {
            channel_id: i64,
            ask: AskUpdate,
            bid: BidUpdate,
            channel_name: String,
            pair: CurrencyPair,
        }

        #[derive(Debug, Deserialize, Clone)]
        #[serde(untagged)]
        enum BookUpdateIntermdiate {
            Ask(Ask),
            Bid(Bid),
            AskBid(AskBid),
        }

        #[derive(Debug, Deserialize, Clone)]
        #[serde(from = "BookUpdateIntermdiate")]
        pub struct BookUpdate {
            channel_id: i64,

            #[serde(flatten, skip_serializing_if = "Option::is_none")]
            ask: Option<AskUpdate>,

            #[serde(flatten, skip_serializing_if = "Option::is_none")]
            bid: Option<BidUpdate>,

            channel_name: String,

            pair: CurrencyPair,
        }

        impl From<BookUpdateIntermdiate> for BookUpdate {
            fn from(from: BookUpdateIntermdiate) -> Self {
                match from {
                    BookUpdateIntermdiate::Ask(Ask {
                        channel_id,
                        ask,
                        channel_name,
                        pair,
                    }) => Self {
                        channel_id,
                        ask: Some(ask),
                        bid: None,
                        channel_name,
                        pair,
                    },
                    BookUpdateIntermdiate::Bid(Bid {
                        channel_id,
                        bid,
                        channel_name,
                        pair,
                    }) => Self {
                        channel_id,
                        ask: None,
                        bid: Some(bid),
                        channel_name,
                        pair,
                    },
                    BookUpdateIntermdiate::AskBid(AskBid {
                        channel_id,
                        ask,
                        bid,
                        channel_name,
                        pair,
                    }) => Self {
                        channel_id,
                        ask: Some(ask),
                        bid: Some(bid),
                        channel_name,
                        pair,
                    },
                }
            }
        }
    }
}

use serde::Deserialize;

pub use book::snapshot::BookSnapshot;
pub use book::update::BookUpdate;
pub use hearbeat::Heartbeat;
pub use ohlc::Ohlc;
pub use spread::Spread;
pub use system_status::SystemStatus;
pub use ticker::Ticker;
pub use trade::Trade;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum Publication {
    Heartbeat(Heartbeat),
    SystemStatus(SystemStatus),
    Ticker(Ticker),
    Ohlc(Ohlc),
    Trade(Trade),
    Spread(Spread),
    BookSnapshot(BookSnapshot),
    BookUpdate(BookUpdate),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heartbeat() {
        let _: Heartbeat = serde_json::from_str(
            r#"
        {
            "event": "heartbeat"
        }
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_system_status() {
        let _: SystemStatus = serde_json::from_str(
            r#"
        {
            "connectionID": 8628615390848610000,
            "event": "systemStatus",
            "status": "online",
            "version": "1.0.0"
        }
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_ticker() {
        let _: Ticker = serde_json::from_str(
            r#"
        [
            0,
            {
                "a": [
                "5525.40000",
                1,
                "1.000"
                ],
                "b": [
                "5525.10000",
                1,
                "1.000"
                ],
                "c": [
                "5525.10000",
                "0.00398963"
                ],
                "h": [
                "5783.00000",
                "5783.00000"
                ],
                "l": [
                "5505.00000",
                "5505.00000"
                ],
                "o": [
                "5760.70000",
                "5763.40000"
                ],
                "p": [
                "5631.44067",
                "5653.78939"
                ],
                "t": [
                11493,
                16267
                ],
                "v": [
                "2634.11501494",
                "3591.17907851"
                ]
            },
            "ticker",
            "BTC/USD"
        ]
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_ohlc() {
        let _: Ohlc = serde_json::from_str(
            r#"
        [
            42,
            [
                "1542057314.748456",
                "1542057360.435743",
                "3586.70000",
                "3586.70000",
                "3586.60000",
                "3586.60000",
                "3586.68894",
                "0.03373000",
                2
            ],
            "ohlc-5",
            "BTC/USD"
        ]
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_trade() {
        let _: Trade = serde_json::from_str(
            r#"
        [
            0,
            [
                [
                    "5541.20000",
                    "0.15850568",
                    "1534614057.321597",
                    "s",
                    "l",
                    ""
                ],
                [
                    "6060.00000",
                    "0.02455000",
                    "1534614057.324998",
                    "b",
                    "m",
                    ""
                ]
            ],
            "trade",
            "BTC/USD"
        ]
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_spread() {
        let _: Spread = serde_json::from_str(
            r#"
        [
            0,
            [
                "5698.40000",
                "5700.00000",
                "1542057299.545897",
                "1.01234567",
                "0.98765432"
            ],
            "spread",
            "BTC/USD"
        ]
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_book_snapshot() {
        let _: BookSnapshot = serde_json::from_str(
            r#"
        [
            0,
            {
                "as": [
                    [
                        "5541.30000",
                        "2.50700000",
                        "1534614248.123678"
                    ],
                    [
                        "5541.80000",
                        "0.33000000",
                        "1534614098.345543"
                    ],
                    [
                        "5542.70000",
                        "0.64700000",
                        "1534614244.654432"
                    ]
                    ],
                    "bs": [
                    [
                        "5541.20000",
                        "1.52900000",
                        "1534614248.765567"
                    ],
                    [
                        "5539.90000",
                        "0.30000000",
                        "1534614241.769870"
                    ],
                    [
                        "5539.50000",
                        "5.00000000",
                        "1534613831.243486"
                    ]
                ]
            },
            "book-100",
            "BTC/USD"
        ]
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_book_update() {
        let payloads = vec![
            r#"[
            1234,
            {
              "a": [
                [
                  "5541.30000",
                  "2.50700000",
                  "1534614248.456738"
                ],
                [
                  "5542.50000",
                  "0.40100000",
                  "1534614248.456738"
                ]
              ],
              "c": "974942666"
            },
            "book-10",
            "XBT/USD"
          ]"#,
            r#"[
            1234,
            {
              "b": [
                [
                  "5541.30000",
                  "0.00000000",
                  "1534614335.345903"
                ]
              ],
              "c": "974942666"
            },
            "book-10",
            "XBT/USD"
          ]"#,
            r#"[
                1234,
                {
                  "a": [
                    [
                      "5541.30000",
                      "2.50700000",
                      "1534614248.456738"
                    ],
                    [
                      "5542.50000",
                      "0.40100000",
                      "1534614248.456738"
                    ]
                  ]
                },
                {
                  "b": [
                    [
                      "5541.30000",
                      "0.00000000",
                      "1534614335.345903"
                    ]
                  ],
                  "c": "974942666"
                },
                "book-10",
                "XBT/USD"
              ]"#,
            r#"[
                1234,
                {
                  "a": [
                    [
                      "5541.30000",
                      "2.50700000",
                      "1534614248.456738",
                      "r"
                    ],
                    [
                      "5542.50000",
                      "0.40100000",
                      "1534614248.456738",
                      "r"
                    ]
                  ],
                  "c": "974942666"
                },
                "book-25",
                "XBT/USD"
              ]"#,
        ];

        for payload in payloads {
            let _ = serde_json::from_str::<BookUpdate>(payload).unwrap();
        }
    }
}
