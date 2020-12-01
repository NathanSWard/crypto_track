use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Error {
    Ok(),
    Err(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Response<R> {
    error: Error,
    result: Option<R>,
}

pub mod trade_history {
    use serde::Deserialize;

    pub type TradeData = crate::message::publication::trade::Data<f64>;

    #[derive(Debug, Clone)]
    pub struct Data {
        pub pair: String, // TODO: CurrencyPair
        pub trade_data: Vec<TradeData>,
    }

    mod data {
        use super::Data;
        use serde::{
            de::{MapAccess, Visitor},
            Deserialize, Deserializer,
        };
        use std::marker::PhantomData;

        struct DataVisitor {
            marker: PhantomData<fn() -> Data>,
        }

        impl DataVisitor {
            fn new() -> Self {
                Self {
                    marker: PhantomData,
                }
            }
        }

        impl<'de> Visitor<'de> for DataVisitor {
            type Value = Data;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("trade data")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                if let Some((pair, trade_data)) = access.next_entry()? {
                    Ok(Data { pair, trade_data })
                } else {
                    Err(serde::de::Error::missing_field(
                        "TradeHistory Data requies a singley key/value pair",
                    ))
                }
            }
        }

        impl<'de> Deserialize<'de> for Data {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_map(DataVisitor::new())
            }
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Result {
        #[serde(flatten)]
        pub data: Data,
        pub last: String,
    }

    pub type TradeHistory = super::Response<Result>;
}

pub mod asset_pairs {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct AssetPairInfo {
        altname: String,
        wsname: Option<String>,
        aclass_base: String,
        base: String,
        aclass_quote: String,
        quote: String,
        lot: String,
        pair_decimals: i64,
        lot_decimals: i64,
        lot_multiplier: i64,
        leverage_buy: Vec<i64>,
        leverage_sell: Vec<i64>,
        fees: Vec<(i64, f64)>,
        fees_maker: Option<Vec<(i64, f64)>>,
        fee_volume_currency: String,
        margin_call: i64,
        margin_stop: i64,
        ordermin: crate::core::KrakenFloat,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Result {
        #[serde(flatten, deserialize_with = "des::deserialize_asset_pair_info")]
        pub data: std::collections::HashMap<String, AssetPairInfo>,
    }

    // custom Deserializer to ignore "DarkData" (e.g. AssetPairs ending with `.d`)
    pub(crate) mod des {
        type Data = std::collections::HashMap<String, super::AssetPairInfo>;
        use serde::{
            de::{MapAccess, Visitor},
            Deserializer,
        };
        use std::marker::PhantomData;

        struct DataVisitor {
            marker: PhantomData<fn() -> Data>,
        }

        impl DataVisitor {
            fn new() -> Self {
                Self {
                    marker: PhantomData,
                }
            }
        }

        impl<'de> Visitor<'de> for DataVisitor {
            type Value = Data;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("trade data")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = Data::with_capacity(access.size_hint().unwrap_or(0));
                loop {
                    let entry = access.next_entry::<String, super::AssetPairInfo>();
                    match entry {
                        Ok(None) => break,
                        Ok(Some((pair, info))) => {
                            if !pair.ends_with(".d") {
                                map.insert(pair, info);
                            }
                        }
                        Err(_) => {}
                    }
                }

                if map.is_empty() {
                    Err(serde::de::Error::missing_field("missing AssetPair map"))
                } else {
                    Ok(map)
                }
            }
        }

        pub(crate) fn deserialize_asset_pair_info<'de, D>(deserializer: D) -> Result<Data, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(DataVisitor::new())
        }
    }

    pub type AssetPairs = super::Response<Result>;
}

pub use asset_pairs::AssetPairs;
pub use trade_history::TradeHistory;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trade_history() {
        let payload = r#"{
            "error":[
               
            ],
            "result":{
               "XXBTZUSD":[
                  [
                     "8552.90000",
                     "0.03190270",
                     1559347203.7998,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.03155529",
                     1559347203.8086,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.00510797",
                     1559347203.9664,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.09047336",
                     1559347203.9789,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.00328738",
                     1559347203.9847,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.00492152",
                     1559347203.9897,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.00201848",
                     1559347203.9937,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.11422068",
                     1559347203.9993,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.00425858",
                     1559347204.071,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.00427679",
                     1559347204.0762,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8552.90000",
                     "0.06381401",
                     1559347204.1662,
                     "s",
                     "m",
                     ""
                  ],
                  [
                     "8579.50000",
                     "0.05379597",
                     1559350785.248,
                     "s",
                     "l",
                     ""
                  ],
                  [
                     "8579.50000",
                     "0.94620403",
                     1559350785.2936,
                     "s",
                     "l",
                     ""
                  ],
                  [
                     "8578.10000",
                     "0.45529068",
                     1559350785.297,
                     "s",
                     "l",
                     ""
                  ]
               ],
               "last":"1559350785297011117"
            }
         }"#;

        let _ = serde_json::from_str::<TradeHistory>(payload).unwrap();
    }
}
