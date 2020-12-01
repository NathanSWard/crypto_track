use crate::core::CurrencyPair;

const KRAKEN_TRADE_HISTORY_URL: &str = "https://api.kraken.com/0/public/Trades";

#[derive(Debug, Clone, TypedBuilder)]
pub struct TradeHistory {
    pub pair: CurrencyPair,

    #[builder(setter(into, strip_option), default)]
    pub since: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl TradeHistory {
    fn as_uri(&self) -> String {
        self.since.map_or_else(
            || {
                format!(
                    "{}?pair={}{}",
                    KRAKEN_TRADE_HISTORY_URL,
                    self.pair.cryptocurrency.abrv(),
                    self.pair.currency.alpha3(),
                )
            },
            |dt| {
                format!(
                    "{}?pair={}{}&since={}",
                    KRAKEN_TRADE_HISTORY_URL,
                    self.pair.cryptocurrency.abrv(),
                    self.pair.currency.alpha3(),
                    dt.timestamp_nanos()
                )
            },
        )
    }

    pub fn url(self) -> String {
        self.as_uri()
    }
}

impl<B> From<TradeHistory> for http::Request<B>
where
    B: Default,
{
    fn from(hist: TradeHistory) -> Self {
        http::Request::get(hist.as_uri())
            .body(B::default())
            .unwrap()
    }
}
