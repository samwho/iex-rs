// TODO: implement batch_request()

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

/// `IEX_URL` is the URL base of IEX API.
const IEX_URL: &str = "https://api.iextrading.com/1.0";

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    // REVIEW(Duration::Date): This should suffice - but as of now, no way to validate input format.
    Date(&'static str),
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
            Duration::Date(date) => format!("date/{}", date),
            Duration::Dynamic => String::from("dynamic"),
        }
    }
}

impl Default for Duration {
    fn default() -> Duration {
        Duration::OneMonth
    }
}

#[derive(PartialEq, Eq)]
/// The `Request` enum type allows for HTTP request matching to the IEX API.
// TODO(Request): Add documentation from IEX website.
pub enum Request<'a> {
    Book {
        symbol: &'a str,
    },
    // REVIEW(CHART): Should `time-series` be implemented even though it forwards
    // towards the same endpoint as `Chart`?
    Chart {
        symbol: &'a str,
        duration: Duration,
        params: Option<Vec<ChartParam>>,
    },
    Company {
        symbol: &'a str,
    },
    DelayedQuote {
        symbol: &'a str,
    },
    Dividends {
        symbol: &'a str,
        duration: Duration,
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
    // REVIEW|TODO(List): add in default displayParameters(?)
    List {
        param: ListParam,
    },
    Logo {
        symbol: &'a str,
    },
    News {
        symbol: &'a str,
        range: Option<i32>,
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
    Splits {
        symbol: &'a str,
        duration: Duration,
    },
    Stats {
        symbol: &'a str,
    },
    Symbols,
    /// IEX Regulation SHO Threshold Securities List
    ThresholdSecurities {
        // REVIEW: may be a good idea to implement a date struct to use here and
        // wrap in the Duration::Date enum variant, or eq - a match statement.
        date: Option<Duration>,
    },
    // TODO(ShortInterest): implement variant.
    VolumeByVenue {
        symbol: &'a str,
    },
}

impl<'a> ToString for Request<'a> {
    fn to_string(&self) -> String {
        match self {
            Request::Book { symbol } => format!("stock/{}/book", symbol),
            Request::Chart {
                symbol,
                duration,
                params,
            } => format!(
                "stock/{}/chart/{}{}",
                symbol,
                duration.to_string(),
                parse_params(params)
            ),
            Request::Company { symbol } => format!("stock/{}/company", symbol),
            Request::DelayedQuote { symbol } => format!("stock/{}/delayed-quote", symbol),
            Request::Dividends { symbol, duration } => {
                format!("stock/{}/dividends/{}", symbol, duration.to_string())
            }
            Request::Earnings { symbol } => format!("stock/{}/earnings", symbol),
            Request::EffectiveSpread { symbol } => format!("stock/{}/effective-spread", symbol),
            Request::Financials { symbol } => format!("stock/{}/financials", symbol),
            Request::List { param } => format!("stock/market/list/{}", param.to_string()),
            Request::Logo { symbol } => format!("stock/{}/logo", symbol),
            Request::News { symbol, range } => format!(
                "stock/{}/news/last/{}",
                symbol,
                range.map(|r| r.to_string()).unwrap_or("".to_string())
            ),
            Request::Ohlc { symbol } => format!("stock/{}/ohlc", symbol),
            Request::Peers { symbol } => format!("stock/{}/peers", symbol),
            // TODO(Request::Previous) It's possible to pass in "market" as an argument here
            // and get one entry for each symbol. We need to handle that
            // scenario.
            // REVIEW(Request::Previous): Regarding ^: should be taken care of
            // due to change in Client's request method signature to serve_json::Value. Let me know if otherwise.
            Request::Previous { symbol } => format!("stock/{}/previous", symbol),
            Request::Price { symbol } => format!("stock/{}/price", symbol),
            Request::Quote { symbol } => format!("stock/{}/quote", symbol),
            Request::Relevant { symbol } => format!("stock/{}/relevant", symbol),
            Request::Stats { symbol } => format!("stock/{}/stats", symbol),
            Request::Splits { symbol, duration } => {
                format!("stock/{}/splits/{}", symbol, duration.to_string())
            }
            Request::Symbols => String::from("/ref-data/symbols"),
            Request::ThresholdSecurities { date } => format!(
                "stock/market/threshold-securities/{}",
                date.unwrap_or(Duration::Date("")).to_string()
            ),
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

/// `Client` acts as a Handler for the `Response` enum.
#[derive(Default)]
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

#[derive(PartialEq, Eq)]
pub enum ListParam {
    MostActive,
    Gainers,
    Losers,
    IexVolume,
    IexPercent,
}

impl ListParam {
    fn to_string(&self) -> String {
        match self {
            ListParam::MostActive => String::from("mostactive"),
            ListParam::Gainers => String::from("gainers"),
            ListParam::Losers => String::from("losers"),
            ListParam::IexVolume => String::from("iexvolume"),
            ListParam::IexPercent => String::from("iexpercent"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ChartParam {
    /// boolean. If true, 1d chart will reset at midnight instead of the default behavior of 9:30am ET.
    Reset(bool),
    /// boolean. If true, runs a polyline simplification using the Douglas-Peucker algorithm. This is useful if plotting sparkline charts.
    Simplify(bool),
    /// number. If passed, chart data will return every Nth element as defined by `Interval`.
    Interval(usize),
    /// boolean. If true, changeOverTime and marketChangeOverTime will be relative to previous day close instead of the first value.
    ChangeFromClose(bool),
    /// number. If passed, chart data will return the last N elements.
    Last(usize),
}

impl ToString for ChartParam {
    fn to_string(&self) -> String {
        match self {
            ChartParam::Reset(res) => format!("chartReset={}", res),
            ChartParam::Simplify(res) => format!("chartSimplify={}", res),
            ChartParam::Interval(res) => format!("chartInterval={}", res),
            ChartParam::ChangeFromClose(res) => format!("changeFromClose={}", res),
            ChartParam::Last(res) => format!("chartLast={}", res),
        }
    }
}

/// parse_params is a function to add a query to a base url.
///
/// # Examples
///
/// ```
/// use iex::{ChartParam, parse_params};
///
/// assert_eq!("?chartReset=true", parse_params(&Some(vec!(ChartParam::Reset(true)))));
/// ```
// REVIEW|QUESTION(): Not sure why &Option is required rather than Option.
pub fn parse_params(params: &Option<Vec<ChartParam>>) -> String {
    if params.is_none() {
        return String::from("");
    }
    let mut result = String::from("?");

    for (i, param) in params.as_ref().unwrap().iter().enumerate() {
        if i != 0 {
            result += "&";
        }
        result += &param.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE(test): Unfortunately rust has no implementation of sub-testing, or at least not aware of as of yet.
    #[test]
    fn test_client_request_book() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Book { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_chart() {
        let client = Client;
        let symbol = "aapl";
        let duration = Duration::OneDay;

        assert!(
            client
                .request(&Request::Chart {
                    symbol,
                    duration,
                    params: None
                })
                .is_ok()
        );
    }

    #[test]
    fn test_client_request_company() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Company { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_delayed_quote() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::DelayedQuote { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_dividends() {
        let client = Client;
        let symbol = "aapl";
        let duration = Duration::OneDay;
        assert!(
            client
                .request(&Request::Dividends { symbol, duration })
                .is_ok()
        );
    }

    #[test]
    fn test_client_request_earnings() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Earnings { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_effective_spread() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::EffectiveSpread { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_financials() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Financials { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_list() {
        let client = Client;

        assert!(
            client
                .request(&Request::List {
                    param: ListParam::Gainers
                })
                .is_ok()
        );
    }

    #[test]
    fn test_client_request_logo() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Logo { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_news() {
        let client = Client;
        let symbol = "aapl";
        assert!(
            client
                .request(&Request::News {
                    symbol,
                    range: None
                })
                .is_ok()
        );
    }

    #[test]
    fn test_client_request_ohlc() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Ohlc { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_peers() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Peers { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_previous() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Previous { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_price() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Price { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_quote() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Quote { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_relevant() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Relevant { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_splits() {
        let client = Client;
        let symbol = "aapl";
        let duration = Duration::OneDay;

        assert!(
            client
                .request(&Request::Splits { symbol, duration })
                .is_ok()
        );
    }

    #[test]
    fn test_client_request_stats() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::Stats { symbol }).is_ok());
    }

    #[test]
    fn test_client_request_symbols() {
        let client = Client;

        assert!(client.request(&Request::Symbols).is_ok());
    }

    #[test]
    fn test_client_request_threshold_securiteis() {
        let client = Client;

        assert!(
            client
                .request(&Request::ThresholdSecurities { date: None })
                .is_ok()
        );
    }

    #[test]
    fn test_client_request_volume_by_venue() {
        let client = Client;
        let symbol = "aapl";

        assert!(client.request(&Request::VolumeByVenue { symbol }).is_ok());
    }
}
