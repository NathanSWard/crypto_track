use crate::cryptocurrency::Cryptocurrency;
use crate::currency::Currency;
use serde::{Deserialize, Serialize};

// THIS IS SO ANNOYING!!! :(
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd)]
#[serde(try_from = "&str", into = "String")]
pub struct KrakenFloat(f64);

impl Into<String> for KrakenFloat {
    fn into(self) -> String {
        format!("{}", self.0)
    }
}

impl std::convert::TryFrom<&str> for KrakenFloat {
    type Error = std::num::ParseFloatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(value.parse::<KrakenFloat>()?)
    }
}

impl std::str::FromStr for KrakenFloat {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let float = s.parse::<f64>()?;
        Ok(Self(float))
    }
}

impl std::ops::Deref for KrakenFloat {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for KrakenFloat {
    fn from(float: f64) -> Self {
        Self(float)
    }
}

impl std::fmt::Debug for KrakenFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for KrakenFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}kf", self.0)
    }
}

unsafe impl Send for KrakenFloat {}
unsafe impl Sync for KrakenFloat {}
impl Unpin for KrakenFloat {}
impl std::panic::UnwindSafe for KrakenFloat {}
impl std::panic::RefUnwindSafe for KrakenFloat {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(into = "String", try_from = "&str")]
pub struct CurrencyPair {
    pub cryptocurrency: Cryptocurrency,
    pub currency: Currency,
}

impl std::convert::TryFrom<&str> for CurrencyPair {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split("/");

        let cryptocurrency = match split.next() {
            Some(s) => s.parse::<Cryptocurrency>()?,
            None => return Err("missing cryptocurrency"),
        };

        let currency = match split.next() {
            Some(s) => s.parse::<Currency>()?,
            None => return Err("missing currency"),
        };

        Ok(Self {
            cryptocurrency,
            currency,
        })
    }
}

impl std::str::FromStr for CurrencyPair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        std::convert::TryFrom::try_from(s)
    }
}

impl From<CurrencyPair> for String {
    fn from(cp: CurrencyPair) -> Self {
        format!("{}/{}", cp.cryptocurrency.abrv(), cp.currency.alpha3())
    }
}

impl std::fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}",
            self.cryptocurrency.abrv(),
            self.currency.alpha3()
        )
    }
}

unsafe impl Send for CurrencyPair {}
unsafe impl Sync for CurrencyPair {}
impl Unpin for CurrencyPair {}
impl std::panic::UnwindSafe for CurrencyPair {}
impl std::panic::RefUnwindSafe for CurrencyPair {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_currency_pair() {
        let invalid = "INVALID";
        let valid = "ETH/USD";

        let cp = invalid.parse::<CurrencyPair>();
        assert!(cp.is_err());

        let cp = valid.parse::<CurrencyPair>().unwrap();
        assert_eq!(cp.cryptocurrency.abrv(), "ETH");
        assert_eq!(cp.currency.alpha3(), "USD");
    }
}
