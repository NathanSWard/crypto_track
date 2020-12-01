use serde::{Deserialize, Serialize};

pub mod subscription_status {
    use crate::core::CurrencyPair;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum SubscriptionStatusEvent {
        SubscriptionStatus,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Subscription {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub depth: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub interval: Option<i64>,

        pub name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Result {
        #[serde(rename = "channelID")]
        ChannelId(i64),

        #[serde(rename = "errorMessage")]
        ErrorMessage(String),
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct SubscriptionStatus {
        event: SubscriptionStatusEvent,

        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        pub result: Option<Result>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel_name: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub reqid: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub pair: Option<CurrencyPair>,

        pub status: String,

        pub subscription: Subscription,
    }
} // mod subscription_status

pub mod order {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(rename_all = "lowercase")]
    pub enum OrderStatus {
        Ok,
        Error,
    }
}

pub mod add_order_status {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum AddOrderStatusEvent {
        AddOrderStatus,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct AddOrderStatus {
        event: AddOrderStatusEvent,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub reqid: Option<i64>,

        pub status: super::order::OrderStatus,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub txid: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub descr: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
    }
}

pub mod cancel_order_status {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum CancelOrderStatusEvent {
        CancelOrderStatus,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct CancelOrderStatus {
        event: CancelOrderStatusEvent,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub reqid: Option<i64>,

        pub status: super::order::OrderStatus,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
    }
}

pub mod cancel_all_status {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    enum CancelAllStatusEvent {
        CancelAllStatus,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct CancelAllStatus {
        event: CancelAllStatusEvent,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub reqid: Option<i64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub count: Option<i64>, // Will kraken send `"count": 0` if it failed???

        pub status: super::order::OrderStatus,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
    }
} // mod cancel_all

pub mod pong {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "snake_case")]
    enum PongEvent {
        Pong,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields)]
    pub struct Pong {
        event: PongEvent,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub reqid: Option<i64>,
    }
}

pub mod error {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(rename_all = "snake_case")]
    enum ErrorEvent {
        Error,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct Error {
        event: ErrorEvent,

        pub error_message: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub reqid: Option<i64>,
    }
}

pub use add_order_status::AddOrderStatus;
pub use cancel_all_status::CancelAllStatus;
pub use cancel_order_status::CancelOrderStatus;
pub use error::Error;
pub use pong::Pong;
pub use subscription_status::SubscriptionStatus;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Response {
    Error(Error),
    SubscriptionStatus(SubscriptionStatus),
    Pong(Pong),
    AddOrderStatus(AddOrderStatus),
    CancelAllStatus(CancelAllStatus),
    CancelOrderStatus(CancelOrderStatus),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pong() {
        let payloads = vec![
            r#"{
            "event": "pong",
            "reqid": 42
          }"#,
            r#"{
            "event": "pong"
          }"#,
        ];
        for payload in payloads {
            let value: Pong = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_error() {
        let payloads = vec![
            r#"{
            "errorMessage": "Malformed request",
            "event": "error"
          }"#,
            r#"{
                "errorMessage":"Exceeded msg rate",
                "event": "error",
                "reqid": 42
              }"#,
        ];
        for payload in payloads {
            let value: Error = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_subscription_status() {
        let payloads = vec![
            r#"{
            "channelID": 10001,
            "channelName": "ticker",
            "event": "subscriptionStatus",
            "pair": "XBT/EUR",
            "status": "subscribed",
            "subscription": {
              "name": "ticker"
            }
          }"#,
            r#"{
            "channelID": 10001,
            "channelName": "ohlc-5",
            "event": "subscriptionStatus",
            "pair": "XBT/EUR",
            "reqid": 42,
            "status": "unsubscribed",
            "subscription": {
              "interval": 5,
              "name": "ohlc"
            }
          }"#,
            r#"{
            "channelName": "ownTrades",
            "event": "subscriptionStatus",
            "status": "subscribed",
            "subscription": {
              "name": "ownTrades"
            }
          }"#,
            r#"{
            "errorMessage": "Subscription depth not supported",
            "event": "subscriptionStatus",
            "pair": "XBT/USD",
            "status": "error",
            "subscription": {
              "depth": 42,
              "name": "book"
            }
          }"#,
        ];
        for payload in payloads {
            let value: SubscriptionStatus = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_add_order_status() {
        let payloads = vec![
            r#"{
            "descr": "buy 0.01770000 XBTUSD @ limit 4000",
            "event": "addOrderStatus",
            "status": "ok",
            "txid": "ONPNXH-KMKMU-F4MR5V"
          }"#,
            r#"{
            "errorMessage": "EOrder:Order minimum not met",
            "event": "addOrderStatus",
            "status": "error"
          }"#,
        ];
        for payload in payloads {
            let value: AddOrderStatus = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_cancel_order_status() {
        let payloads = vec![
            r#"{
            "event": "cancelOrderStatus",
            "status": "ok"
          }"#,
            r#"{
            "errorMessage": "EOrder:Unknown order",
            "event": "cancelOrderStatus",
            "status": "error"
          }"#,
        ];
        for payload in payloads {
            let value: CancelOrderStatus = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }

    #[test]
    fn test_cancel_all_status() {
        let payloads = vec![
            r#"{
            "count": 2,
            "event": "cancelAllStatus",
            "status": "ok"
          }"#,
            r#"{
            "errorMessage": "EOrder:Unknown order",
            "event": "cancelAllStatus",
            "status": "error"
          }"#,
        ];
        for payload in payloads {
            let value: CancelAllStatus = serde_json::from_str(payload).unwrap();
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json::parse(payload).unwrap(), json::parse(&json).unwrap());
        }
    }
}
