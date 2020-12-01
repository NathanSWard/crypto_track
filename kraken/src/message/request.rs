pub mod ping {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum PingEvent {
        Ping,
    }

    impl Default for PingEvent {
        fn default() -> Self {
            PingEvent::Ping
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
    #[serde(deny_unknown_fields)]
    pub struct Ping {
        #[builder(setter(skip), default)]
        event: PingEvent,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub reqid: Option<i64>,
    }
}

pub mod subscribe {
    use crate::core::CurrencyPair;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum SubscribeEvent {
        Subscribe,
    }

    impl Default for SubscribeEvent {
        fn default() -> Self {
            SubscribeEvent::Subscribe
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
    #[serde(deny_unknown_fields)]
    pub struct Subscription {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub depth: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub interval: Option<i64>,

        #[builder(setter(into))]
        pub name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub snapshot: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub token: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
    #[serde(deny_unknown_fields)]
    pub struct Subscribe {
        #[builder(setter(skip), default)]
        event: SubscribeEvent,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub reqid: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub pair: Option<Vec<CurrencyPair>>,

        #[builder(setter(into))]
        pub subscription: Subscription,
    }
} // mod subscribe

pub mod unsubscribe {
    use crate::core::CurrencyPair;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum UnsubscribeEvent {
        Unsubscribe,
    }

    impl Default for UnsubscribeEvent {
        fn default() -> Self {
            UnsubscribeEvent::Unsubscribe
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
    #[serde(deny_unknown_fields)]
    pub struct Subscription {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub depth: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub interval: Option<i64>,

        #[builder(setter(into))]
        pub name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub token: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum UnsubscribeFrom {
        #[serde(rename = "channelID")]
        ChannelID(i32),

        #[serde(rename = "pair")]
        Pair(Vec<CurrencyPair>),
    }

    #[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
    #[serde(deny_unknown_fields)]
    pub struct Unsubscribe {
        #[builder(setter(skip), default)]
        event: UnsubscribeEvent,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub reqid: Option<i64>,

        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub from: Option<UnsubscribeFrom>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub subscription: Option<Subscription>,
    }
}

pub mod add_order {
    use crate::core::{CurrencyPair, KrakenFloat};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum AddOrderEvent {
        AddOrder,
    }

    impl Default for AddOrderEvent {
        fn default() -> Self {
            AddOrderEvent::AddOrder
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
    #[serde(deny_unknown_fields)]
    pub struct AddOrder {
        #[builder(setter(skip), default)]
        event: AddOrderEvent,

        #[builder(setter(into))]
        pub token: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub reqid: Option<i64>,

        #[builder(setter(into))]
        pub ordertype: String, // TODO CUSTOM TYPE

        #[serde(rename = "type")]
        #[builder(setter(into))]
        pub ty: String, // TODO CUSTOM TYPE

        pub pair: CurrencyPair,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub price: Option<KrakenFloat>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub price2: Option<KrakenFloat>,

        #[builder(setter(into))]
        pub volume: KrakenFloat,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub leverage: Option<KrakenFloat>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub oflags: Option<String>, // TODO CUSTOM TYPE

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub starttm: Option<String>, // TODO CUSTOM TYPE

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub expiretm: Option<String>, // TODO CUSTOM TYPE

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub userref: Option<String>, // TODO CUSTOM TYPE

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub validate: Option<String>,

        #[serde(rename = "close[ordertype]", skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub close_ordertype: Option<String>,

        #[serde(rename = "close[price]", skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub close_price: Option<String>,

        #[serde(rename = "close[price2]", skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub close_price2: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub trading_agreement: Option<String>,
    }
}

pub mod cancel_order {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum CancelOrderEvent {
        CancelOrder,
    }

    impl Default for CancelOrderEvent {
        fn default() -> Self {
            CancelOrderEvent::CancelOrder
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
    #[serde(deny_unknown_fields)]
    pub struct CancelOrder {
        #[builder(setter(skip), default)]
        event: CancelOrderEvent,

        #[builder(setter(into))]
        pub token: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub reqid: Option<i64>,

        #[builder(setter(into))]
        pub txid: Vec<String>,
    }
}

pub mod cancel_all {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum CancelAllEvent {
        CancelAll,
    }

    impl Default for CancelAllEvent {
        fn default() -> Self {
            CancelAllEvent::CancelAll
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
    #[serde(deny_unknown_fields)]
    pub struct CancelAll {
        #[builder(setter(skip), default)]
        event: CancelAllEvent,

        #[builder(setter(into))]
        pub token: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[builder(setter(into, strip_option), default)]
        pub reqid: Option<i64>,
    }
}

pub use add_order::AddOrder;
pub use cancel_all::CancelAll;
pub use cancel_order::CancelOrder;
pub use ping::Ping;
pub use subscribe::Subscribe;
pub use unsubscribe::Unsubscribe;

pub trait Request {}
impl Request for Ping {}
impl Request for Subscribe {}
impl Request for Unsubscribe {}
impl Request for AddOrder {}
impl Request for CancelOrder {}
impl Request for CancelAll {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::CurrencyPair;
    use json::object;

    #[test]
    fn test_ping() {
        let ping = Ping::builder().reqid(42).build();
        let json = object! {
            event: "ping",
            reqid: 42,
        }
        .to_string();
        assert_eq!(serde_json::to_string(&ping).unwrap(), json);

        let payloads = vec![
            r#"{
                "event": "ping",
                "reqid": 42
              }"#,
            r#"{
                "event": "ping"
              }"#,
        ];
        for payload in payloads {
            let value: Ping = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_subscribe() {
        let subscription = subscribe::Subscription::builder()
            .depth(1)
            .interval(1)
            .name("ticker")
            .snapshot(false)
            .token("0000")
            .build();

        let subscribe = Subscribe::builder()
            .reqid(1)
            .pair(vec!["BTC/USD".parse::<CurrencyPair>().unwrap()])
            .subscription(subscription)
            .build();

        let json = object! {
            event: "subscribe",
            reqid: 1,
            pair: [
                "BTC/USD"
            ],
            subscription: {
                depth: 1,
                interval: 1,
                name: "ticker",
                snapshot: false,
                token: "0000"
            }
        }
        .to_string();
        assert_eq!(serde_json::to_string(&subscribe).unwrap(), json);

        let payloads = vec![
            r#"{
                "event": "subscribe",
                "pair": [
                  "XBT/USD",
                  "XBT/EUR"
                ],
                "subscription": {
                  "name": "ticker"
                }
              }"#,
            r#"{
                "event": "subscribe",
                "pair": [
                  "XBT/EUR"
                ],
                "subscription": {
                  "interval": 5,
                  "name": "ohlc"
                }
              }"#,
            r#"{
                "event": "subscribe",
                "subscription": {
                  "name": "ownTrades",
                  "token": "WW91ciBhdXRoZW50aWNhdGlvbiB0b2tlbiBnb2VzIGhlcmUu"
                }
              }"#,
        ];
        for payload in payloads {
            let value: Subscribe = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_unsubscribe() {
        let subscription = unsubscribe::Subscription::builder()
            .depth(1)
            .interval(1)
            .name("ticker")
            .token("0000")
            .build();

        let unsubscribe = unsubscribe::Unsubscribe::builder()
            .reqid(1)
            .from(unsubscribe::UnsubscribeFrom::Pair(vec!["BTC/USD"
                .parse::<CurrencyPair>()
                .unwrap()]))
            .subscription(subscription)
            .build();

        let json = object! {
            event: "unsubscribe",
            reqid: 1,
            pair: [
                "BTC/USD"
            ],
            subscription: {
                depth: 1,
                interval: 1,
                name: "ticker",
                token: "0000"
            }
        }
        .to_string();
        assert_eq!(serde_json::to_string(&unsubscribe).unwrap(), json);

        let payloads = vec![
            r#"{
                "event": "unsubscribe",
                "pair": [
                  "XBT/EUR",
                  "XBT/USD"
                ],
                "subscription": {
                  "name": "ticker"
                }
              }"#,
            r#"{
                "channelID": 10001,
                "event": "unsubscribe"
              }"#,
            r#"{
                "event": "unsubscribe",
                "subscription": {
                  "name": "ownTrades",
                  "token": "WW91ciBhdXRoZW50aWNhdGlvbiB0b2tlbiBnb2VzIGhlcmUu"
                }
              }"#,
        ];
        for payload in payloads {
            let value: Unsubscribe = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_add_order() {
        let add_order = AddOrder::builder()
            .token("0000")
            .reqid(1)
            .ordertype("a")
            .ty("a")
            .pair("BTC/USD".parse::<CurrencyPair>().unwrap())
            .price(1.0)
            .price2(1.0)
            .volume(1.0)
            .leverage(1.0)
            .oflags("a")
            .starttm("a")
            .expiretm("a")
            .userref("a")
            .validate("a")
            .close_ordertype("a")
            .close_price("a")
            .close_price2("a")
            .trading_agreement("a")
            .build();

        let json = object! {
            event: "addOrder",
            token: "0000",
            reqid: 1,
            ordertype: "a",
            "type": "a",
            pair: "BTC/USD",
            price: "1",
            price2: "1",
            volume: "1",
            leverage: "1",
            oflags: "a",
            starttm: "a",
            expiretm: "a",
            userref: "a",
            validate: "a",
            "close[ordertype]": "a",
            "close[price]": "a",
            "close[price2]": "a",
            trading_agreement: "a"
        }
        .to_string();
        assert_eq!(serde_json::to_string(&add_order).unwrap(), json);

        let payloads = vec![
            r#"{
            "event": "addOrder",
            "ordertype": "limit",
            "pair": "XBT/USD",
            "price": "9000",
            "token": "0000000000000000000000000000000000000000",
            "type": "buy",
            "volume": "10"
          }"#,
            r#"{
                "close[ordertype]": "limit",
                "close[price]": "9100",
                "event": "addOrder",
                "ordertype": "limit",
                "pair": "XBT/USD",
                "price": "9000",
                "token": "0000000000000000000000000000000000000000",
                "type": "buy",
                "volume": "10"
              }"#,
        ];
        for payload in payloads {
            let value: AddOrder = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_cancel_order() {
        let cancel_order = CancelOrder::builder()
            .token("0000")
            .reqid(1)
            .txid(vec!["0000".into()])
            .build();

        let json = object! {
            event: "cancelOrder",
            token: "0000",
            reqid: 1,
            txid: [
                "0000"
            ]
        }
        .to_string();
        assert_eq!(serde_json::to_string(&cancel_order).unwrap(), json);

        let payloads = vec![
            r#"{
                "event": "cancelOrder",
                "token": "0000000000000000000000000000000000000000",
                "txid": [
                  "OGTT3Y-C6I3P-XRI6HX",
                  "OGTT3Y-C6I3P-X2I6HX"
                ]
              }"#,
        ];
        for payload in payloads {
            let value: CancelOrder = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_cancel_all() {
        let cancel_all = CancelAll::builder().token("0000").reqid(1).build();

        let json = object! {
            event: "cancelAll",
            token: "0000",
            reqid: 1
        }
        .to_string();
        assert_eq!(serde_json::to_string(&cancel_all).unwrap(), json);

        let payloads = vec![
            r#"{
                "event": "cancelAll",
                "token": "0000000000000000000000000000000000000000"
              }"#,
        ];
        for payload in payloads {
            let value: CancelAll = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }
}
