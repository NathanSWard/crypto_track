use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum Cryptocurrency {
    ZRX = 0,
    LEND,
    ALGO,
    AR,
    REP,
    BAL,
    BNT,
    BAT,
    BTC,
    XBT,
    BCH,
    BRICK,
    ADA,
    LINK,
    COMP,
    ATOM,
    CRV,
    DAI,
    DASH,
    DOGE,
    ENJ,
    EOS,
    ETH,
    ETC,
    FIL,
    FLOW,
    GNO,
    HNS,
    HBAR,
    ICX,
    KAVA,
    KSM,
    KNC,
    LSK,
    LTC,
    MKR,
    MLN,
    XMR,
    MOON,
    NANO,
    OMG,
    OXT,
    PAXG,
    DOT,
    QTUM,
    XRP,
    SC,
    XLM,
    STORJ,
    SNX,
    USDT,
    XTZ,
    TRX,
    WAVES,
    YFI,
    ZEC,
}

impl Cryptocurrency {
    pub fn abrv(&self) -> &'static str {
        CRYPTOCURRENCY_DATA[*self as usize].abrv
    }

    pub fn name(&self) -> &'static str {
        CRYPTOCURRENCY_DATA[*self as usize].name
    }

    pub unsafe fn from_str_unchecked<S: AsRef<str>>(s: S) -> Self {
        CRYPTOCURRENCY_DATA
            .iter()
            .find(|data| data.abrv == s.as_ref())
            .unwrap()
            .val
    }
}

impl std::str::FromStr for Cryptocurrency {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        std::convert::TryFrom::try_from(s)
    }
}

impl std::convert::TryFrom<&str> for Cryptocurrency {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(data) = CRYPTOCURRENCY_DATA.iter().find(|data| data.abrv == value) {
            Ok(data.val)
        } else {
            Err("Invalid cryptocurrency")
        }
    }
}

impl From<Cryptocurrency> for &str {
    fn from(c: Cryptocurrency) -> Self {
        c.abrv()
    }
}

impl std::fmt::Display for Cryptocurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abrv())
    }
}

unsafe impl Send for Cryptocurrency {}
unsafe impl Sync for Cryptocurrency {}
impl Unpin for Cryptocurrency {}
impl std::panic::UnwindSafe for Cryptocurrency {}
impl std::panic::RefUnwindSafe for Cryptocurrency {}

pub struct CryptocurrencyData {
    val: Cryptocurrency,
    abrv: &'static str,
    name: &'static str,
}

const CRYPTOCURRENCY_DATA: [CryptocurrencyData; 56] = [
    CryptocurrencyData {
        val: Cryptocurrency::ZRX,
        abrv: "ZRX",
        name: "0x",
    },
    CryptocurrencyData {
        val: Cryptocurrency::LEND,
        abrv: "LEND",
        name: "Aave",
    },
    CryptocurrencyData {
        val: Cryptocurrency::ALGO,
        abrv: "ALGO",
        name: "Algorand",
    },
    CryptocurrencyData {
        val: Cryptocurrency::AR,
        abrv: "AR",
        name: "Arweave",
    },
    CryptocurrencyData {
        val: Cryptocurrency::REP,
        abrv: "REP",
        name: "Augur",
    },
    CryptocurrencyData {
        val: Cryptocurrency::BAL,
        abrv: "BAL",
        name: "Balancer",
    },
    CryptocurrencyData {
        val: Cryptocurrency::BNT,
        abrv: "BNT",
        name: "Bancor",
    },
    CryptocurrencyData {
        val: Cryptocurrency::BAT,
        abrv: "BAT",
        name: "Basic Attention Token",
    },
    CryptocurrencyData {
        val: Cryptocurrency::BTC,
        abrv: "BTC",
        name: "Bitcoin",
    },
    CryptocurrencyData {
        val: Cryptocurrency::XBT,
        abrv: "XBT",
        name: "Bitcoin",
    },
    CryptocurrencyData {
        val: Cryptocurrency::BCH,
        abrv: "BCH",
        name: "Bitcoin Cash",
    },
    CryptocurrencyData {
        val: Cryptocurrency::BRICK,
        abrv: "BRICK",
        name: "Brick",
    },
    CryptocurrencyData {
        val: Cryptocurrency::ADA,
        abrv: "ADA",
        name: "Cardano",
    },
    CryptocurrencyData {
        val: Cryptocurrency::LINK,
        abrv: "LINK",
        name: "Chainlink",
    },
    CryptocurrencyData {
        val: Cryptocurrency::COMP,
        abrv: "COMP",
        name: "Compound",
    },
    CryptocurrencyData {
        val: Cryptocurrency::ATOM,
        abrv: "ATOM",
        name: "Cosmos",
    },
    CryptocurrencyData {
        val: Cryptocurrency::CRV,
        abrv: "CRV",
        name: "Curve",
    },
    CryptocurrencyData {
        val: Cryptocurrency::DAI,
        abrv: "DAI",
        name: "Dai",
    },
    CryptocurrencyData {
        val: Cryptocurrency::DASH,
        abrv: "DASH",
        name: "Dash",
    },
    CryptocurrencyData {
        val: Cryptocurrency::DOGE,
        abrv: "DOGE",
        name: "Dogecoin",
    },
    CryptocurrencyData {
        val: Cryptocurrency::ENJ,
        abrv: "ENJ",
        name: "Enjin",
    },
    CryptocurrencyData {
        val: Cryptocurrency::EOS,
        abrv: "EOS",
        name: "EOSIO",
    },
    CryptocurrencyData {
        val: Cryptocurrency::ETH,
        abrv: "ETH",
        name: "Ethereum",
    },
    CryptocurrencyData {
        val: Cryptocurrency::ETC,
        abrv: "ETC",
        name: "Ethereum Classic",
    },
    CryptocurrencyData {
        val: Cryptocurrency::FIL,
        abrv: "FIL",
        name: "Filecoin",
    },
    CryptocurrencyData {
        val: Cryptocurrency::FLOW,
        abrv: "FLOW",
        name: "Flow",
    },
    CryptocurrencyData {
        val: Cryptocurrency::GNO,
        abrv: "GNO",
        name: "Gnosis",
    },
    CryptocurrencyData {
        val: Cryptocurrency::HNS,
        abrv: "HNS",
        name: "Handshake",
    },
    CryptocurrencyData {
        val: Cryptocurrency::HBAR,
        abrv: "HBAR",
        name: "Hedera Hashgraph",
    },
    CryptocurrencyData {
        val: Cryptocurrency::ICX,
        abrv: "ICX",
        name: "Icon",
    },
    CryptocurrencyData {
        val: Cryptocurrency::KAVA,
        abrv: "KAVA",
        name: "Kava",
    },
    CryptocurrencyData {
        val: Cryptocurrency::KSM,
        abrv: "KSM",
        name: "Kusama",
    },
    CryptocurrencyData {
        val: Cryptocurrency::KNC,
        abrv: "KNC",
        name: "Kyber Network",
    },
    CryptocurrencyData {
        val: Cryptocurrency::LSK,
        abrv: "LSK",
        name: "Lisk",
    },
    CryptocurrencyData {
        val: Cryptocurrency::LTC,
        abrv: "LTC",
        name: "Litecoin",
    },
    CryptocurrencyData {
        val: Cryptocurrency::MKR,
        abrv: "MKR",
        name: "MakerDAO",
    },
    CryptocurrencyData {
        val: Cryptocurrency::MLN,
        abrv: "MLN",
        name: "Melon",
    },
    CryptocurrencyData {
        val: Cryptocurrency::XMR,
        abrv: "XMR",
        name: "Monero",
    },
    CryptocurrencyData {
        val: Cryptocurrency::MOON,
        abrv: "MOON",
        name: "Moon",
    },
    CryptocurrencyData {
        val: Cryptocurrency::NANO,
        abrv: "NANO",
        name: "Nano",
    },
    CryptocurrencyData {
        val: Cryptocurrency::OMG,
        abrv: "OMG",
        name: "OmiseGo",
    },
    CryptocurrencyData {
        val: Cryptocurrency::OXT,
        abrv: "OXT",
        name: "Orchid",
    },
    CryptocurrencyData {
        val: Cryptocurrency::PAXG,
        abrv: "PAXG",
        name: "PAX Gold",
    },
    CryptocurrencyData {
        val: Cryptocurrency::DOT,
        abrv: "DOT",
        name: "Polkadot",
    },
    CryptocurrencyData {
        val: Cryptocurrency::QTUM,
        abrv: "QTUM",
        name: "Qtum",
    },
    CryptocurrencyData {
        val: Cryptocurrency::XRP,
        abrv: "XRP",
        name: "Ripple",
    },
    CryptocurrencyData {
        val: Cryptocurrency::SC,
        abrv: "SC",
        name: "Siacoin",
    },
    CryptocurrencyData {
        val: Cryptocurrency::XLM,
        abrv: "XLM",
        name: "Stellar",
    },
    CryptocurrencyData {
        val: Cryptocurrency::STORJ,
        abrv: "STORJ",
        name: "Storj",
    },
    CryptocurrencyData {
        val: Cryptocurrency::SNX,
        abrv: "SNX",
        name: "Synthetix",
    },
    CryptocurrencyData {
        val: Cryptocurrency::USDT,
        abrv: "USDT",
        name: "Tether",
    },
    CryptocurrencyData {
        val: Cryptocurrency::XTZ,
        abrv: "XTZ",
        name: "Tezos",
    },
    CryptocurrencyData {
        val: Cryptocurrency::TRX,
        abrv: "TRX",
        name: "Tron",
    },
    CryptocurrencyData {
        val: Cryptocurrency::WAVES,
        abrv: "WAVES",
        name: "Waves",
    },
    CryptocurrencyData {
        val: Cryptocurrency::YFI,
        abrv: "YFI",
        name: "yEarn",
    },
    CryptocurrencyData {
        val: Cryptocurrency::ZEC,
        abrv: "ZEC",
        name: "Zcash",
    },
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cyprocurrencies() {
        for (i, data) in CRYPTOCURRENCY_DATA.iter().enumerate() {
            assert_eq!(i, data.val as usize);
        }
    }
}
