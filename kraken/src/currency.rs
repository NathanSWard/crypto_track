use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Copy, Clone)]
#[serde(try_from = "&str", into = "&str")]
pub struct Currency(usize);

impl Currency {
    pub fn alpha3(&self) -> &'static str {
        iso4217::all()[self.0].alpha3
    }

    pub fn countries(&self) -> &'static [&'static str] {
        iso4217::all()[self.0].countries
    }

    pub fn exp(&self) -> i8 {
        iso4217::all()[self.0].exp
    }

    pub fn name(&self) -> &'static str {
        iso4217::all()[self.0].name
    }

    pub fn num(&self) -> &'static str {
        iso4217::all()[self.0].num
    }

    pub unsafe fn from_str_unchecked<S: AsRef<str>>(s: S) -> Self {
        Self(
            iso4217::all()
                .iter()
                .enumerate()
                .find(|(_, curr)| curr.alpha3 == s.as_ref())
                .unwrap()
                .0,
        )
    }
}

impl std::fmt::Debug for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::convert::TryFrom<&str> for Currency {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        iso4217::all()
            .iter()
            .enumerate()
            .find(|(_, curr)| curr.alpha3 == s)
            .map(|(idx, _)| Currency(idx))
            .ok_or("Invalid currency")
    }
}

impl std::str::FromStr for Currency {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        std::convert::TryFrom::try_from(s)
    }
}

impl From<Currency> for &str {
    fn from(c: Currency) -> Self {
        c.alpha3()
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.alpha3())
    }
}

unsafe impl Send for Currency {}
unsafe impl Sync for Currency {}
impl Unpin for Currency {}
impl std::panic::UnwindSafe for Currency {}
impl std::panic::RefUnwindSafe for Currency {}
