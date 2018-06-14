#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde_json::Value;
pub mod types;

use failure::Error;
use std::result;

const IEX_URL: &'static str = "https://api.iextrading.com/1.0";

pub type Result<T> = result::Result<T, Error>;

pub enum Duration {
    FiveYears,
    TwoYears,
    OneYear,
    YearToDate,
    SixMonths,
    ThreeMonths,
    OneMonth,
    OneDay,
    // TODO: Date returns a different JSON structure to the rest of the duration parameters. We'll
    // need to make a different function to support it.
    // Date(&'a str),
    Dynamic,
}

impl ToString for Duration {
    fn to_string(&self) -> String {
        match self {
            Duration::FiveYears => String::from("5y"),
            Duration::TwoYears => String::from("2y"),
            Duration::OneYear => String::from("1y"),
            Duration::YearToDate => String::from("ytd"),
            Duration::SixMonths => String::from("6m"),
            Duration::ThreeMonths => String::from("3m"),
            Duration::OneMonth => String::from("1m"),
            Duration::OneDay => String::from("1d"),
            Duration::Dynamic => String::from("dynamic"),
        }
    }
}

impl Default for Duration {
    fn default() -> Duration {
        Duration::OneMonth
    }
}

/// The `Request` enum type allows for care-free request matching to the IEX API.
// TODO(Request) - implement chart & duration variants.
pub enum Request<'a> {
    Book {
        symbol: &'a str,
    },
    Company {
        symbol: &'a str,
    },
    DelayedQuote {
        symbol: &'a str,
    },
    Earnings {
        symbol: &'a str,
    },
    EffectiveSpread {
        symbol: &'a str,
    },
    Financials {
        symbol: &'a str,
    },
    Stats {
        symbol: &'a str,
    },
    Logo {
        symbol: &'a str,
    },
    News {
        symbol: &'a str,
        range: Option<usize>,
    },
    Ohlc {
        symbol: &'a str,
    },
    Peers {
        symbol: &'a str,
    },
    Previous {
        symbol: &'a str,
    },
    Price {
        symbol: &'a str,
    },
    Quote {
        symbol: &'a str,
    },
    Relevant {
        symbol: &'a str,
    },
    VolumeByVenue {
        symbol: &'a str,
    },
}

impl<'a> ToString for Request<'a> {
    fn to_string(&self) -> String {
        match self {
            Request::Book { symbol } => format!("stock/{}/book", symbol),
            Request::Company { symbol } => format!("stock/{}/company", symbol),
            Request::DelayedQuote { symbol } => format!("stock/{}/delayed-quote", symbol),
            Request::Earnings { symbol } => format!("stock/{}/earnings", symbol),
            Request::EffectiveSpread { symbol } => format!("stock/{}/effective-spread", symbol),
            Request::Financials { symbol } => format!("stock/{}/financials", symbol),
            Request::Stats { symbol } => format!("stock/{}/stats", symbol),
            Request::Logo { symbol } => format!("stock/{}/logo", symbol),
            Request::News { symbol, range } => {
                format!("stock/{}/news/last/{}", symbol, range.expect(""))
            }
            Request::Ohlc { symbol } => format!("stock/{}/ohlc", symbol),
            Request::Peers { symbol } => format!("stock/{}/peers", symbol),
            // TODO(Request::Previous) It's possible to pass in "market" as an argument here
            // and get one entry for each symbol. We need to handle that
            // scenario.
            // Comment(Jake) Regarding ^: should be taken care of
            // due to change in method signature to serve_json::Value. Let me know if otherwise.
            Request::Previous { symbol } => format!("stock/{}/previous", symbol),
            Request::Price { symbol } => format!("stock/{}/price", symbol),
            Request::Quote { symbol } => format!("stock/{}/quote", symbol),
            Request::Relevant { symbol } => format!("stock/{}/relevant", symbol),
            Request::VolumeByVenue { symbol } => format!("stock/{}/volume-by-venue", symbol),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response(pub Value);

impl Response {
    pub fn try_into<T>(self) -> Result<T>
    // TEMP(Response): Keep until try_from trait becomes stable rust feature.
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        Ok(serde_json::from_value(self.0)?)
    }
}

/// Client acts as a ResponseHandler.
pub struct Client;

impl Client {
    pub fn new() -> Self {
        Client
    }

    pub fn request(&self, req: &Request) -> Result<Response> {
        let url = format!(
            "{base}/{endpoint}",
            base = IEX_URL,
            endpoint = req.to_string()
        );
        Ok(reqwest::get(&url)?.json()?)
    }
}

// #[derive(Serialize, Debug, Builder)]
// #[builder(setter(into))]
// #[serde(rename_all = "camelCase")]
// pub struct ChartParams {
//     #[builder(default)]
//     pub chart_reset: Option<bool>,
//     #[builder(default)]
//     pub chart_simplify: Option<bool>,
//     #[builder(default)]
//     pub chart_interval: Option<i64>,
// }

// impl Default for ChartParams {
//     fn default() -> ChartParams {
//         ChartParamsBuilder::default().build().unwrap()
//     }
// }

// fn get_with_params<R, P>(&self, path: &str, params: P) -> Result<R>
// where
//     R: serde::de::DeserializeOwned,
//     P: serde::ser::Serialize,
// {
//     let uri = format!("{}{}", "https://api.iextrading.com/1.0", path);
//     let res = self.http
//         .get(&uri)
//         .query(&params)
//         .send()?
//         .error_for_status()?;
//     Ok(serde_json::from_reader(res)?)
// }

// pub fn chart(&self, symbol: &str, duration: Duration) -> Result<Vec<ChartDataPoint>> {
//     self.chart_with_params(symbol, duration, ChartParams::default())
// }

// pub fn chart_with_params(
//     &self,
//     symbol: &str,
//     duration: Duration,
//     params: ChartParams,
// ) -> Result<Vec<ChartDataPoint>> {
//     let path = format!("/stock/{}/chart/{}", symbol, duration.to_string());
//     self.get_with_params(&path, params)
// }

// pub fn dividends(&self, symbol: &str, duration: Duration) -> Result<Vec<Dividend>> {
//     self.get(&format!(
//         "/stock/{}/dividends/{}",
//         symbol,
//         duration.to_string()
//     ))
// }

// pub fn financials(&self, symbol: &str) -> Result<Financials> {
//     self.get(&format!("/stock/{}/financials", symbol))
// }

// pub fn iex_regulation_sho_threshold_securities_list(
//     &self,
//     date: Option<&str>,
// ) -> Result<Vec<IEXRegulationSHOThresholdSecurity>> {
//     self.get(&format!(
//         "/stock/market/threshold-securities/{}",
//         date.unwrap_or("")
//     ))
// }

// pub fn iex_short_interest_list(
//     &self,
//     symbol: Option<&str>,
//     date: Option<&str>,
// ) -> Result<Vec<IEXShortInterest>> {
//     self.get(&format!(
//         "/stock/{}/short-interest/{}",
//         symbol.unwrap_or("market"),
//         date.unwrap_or("")
//     ))
// }

// pub fn list(&self, list: &str) -> Result<Vec<Quote>> {
//     self.get(&format!("/stock/market/list/{}", list))
// }

// pub fn splits(&self, symbol: &str, duration: Duration) -> Result<Vec<Split>> {
//     self.get(&format!(
//         "/stock/{}/splits/{}",
//         symbol,
//         duration.to_string()
//     ))
// }

// pub fn time_series(&self, symbol: &str, duration: Duration) -> Result<Vec<ChartDataPoint>> {
//     self.time_series_with_params(symbol, duration, ChartParams::default())
// }

// pub fn time_series_with_params(
//     &self,
//     symbol: &str,
//     duration: Duration,
//     params: ChartParams,
// ) -> Result<Vec<ChartDataPoint>> {
//     self.chart_with_params(symbol, duration, params)
// }

// pub fn symbols(&self) -> Result<Vec<Symbol>> {
//     self.get("/ref-data/symbols")
// }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_request() {
        let client = Client;
        assert!(client.request(&Request::Book { symbol: "aapl" }).is_ok());
        // TODO(test_client_request()) implement other Request cases.
    }

    // #[test]
    // fn chart() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(iex.chart("aapl", ::Duration::default()).is_ok());
    // }

    // #[test]
    // fn dividends() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(iex.dividends("aapl", ::Duration::default()).is_ok());
    // }

    // #[test]
    // fn iex_regulation_sho_threshold_securities_list() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(
    //         iex.iex_regulation_sho_threshold_securities_list(Some("sample"))
    //             .is_ok()
    //     );
    // }

    // #[test]
    // fn iex_short_interest_list() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(iex.iex_short_interest_list(None, None).is_ok());
    // }

    // #[test]
    // fn list() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(iex.list("gainers").is_ok());
    // }

    // #[test]
    // fn splits() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(iex.splits("aapl", ::Duration::default()).is_ok());
    // }

    // #[test]
    // fn time_series() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(iex.time_series("aapl", ::Duration::default()).is_ok());
    // }

    // #[test]
    // fn volume_by_venue() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(iex.volume_by_venue("aapl").is_ok());
    // }

    // #[test]
    // fn symbols() {
    //     let iex = ::IexClient::new().unwrap();
    //     assert!(iex.symbols().is_ok());
    // }
}
