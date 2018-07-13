#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate byteorder;
extern crate chrono;

use serde_json::Value;

mod endpoints;
mod types;
pub mod iextp;

pub use self::endpoints::*;
pub use self::types::*;

use failure::Error;
use std::result;

pub type Result<T> = result::Result<T, Error>;

/// `IEX_URL` is the URL base of IEX API.
const IEX_URL: &str = "https://api.iextrading.com/1.0";

/// `Client` acts as a Handler for the `Response` enum.
#[derive(Default)]
pub struct Client;

impl Client {
    pub fn new() -> Self {
        Client
    }

    /// stocks_request is the main entry-point to the IEX Stocks API.
    pub fn stocks_request<S>(&self, symbol: S, req: StocksEndpoint) -> Result<Response>
    where
        S: Into<String>,
    {
        let url = format!(
            "{base}/stock/{symbol}/{endpoint}",
            base = IEX_URL,
            symbol = symbol.into(),
            endpoint = req.to_endpoint()
        );

        Ok(reqwest::get(&url)?.json()?)
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

pub trait Endpoint {
    fn to_endpoint(self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    static CLIENT: Client = Client;
    #[allow(non_upper_case_globals)]
    static symbol: &'static str = "aapl";
    #[allow(non_upper_case_globals)]
    static duration: Duration = Duration::OneDay;

    #[test]
    fn client_request_book() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Book).is_ok());
    }

    #[test]
    fn client_request_chart() {
        assert!(
            CLIENT
                .stocks_request(
                    symbol,
                    StocksEndpoint::Chart {
                        duration,
                        params: None
                    }
                )
                .is_ok()
        );
    }

    #[test]
    fn client_request_company() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::Company)
                .is_ok()
        );
    }

    #[test]
    fn client_request_delayed_quote() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::DelayedQuote)
                .is_ok()
        );
    }

    #[test]
    fn client_request_dividends() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::Dividends { duration })
                .is_ok()
        );
    }

    #[test]
    fn client_request_earnings() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::Earnings)
                .is_ok()
        );
    }

    #[test]
    fn client_request_effective_spread() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::EffectiveSpread)
                .is_ok()
        );
    }

    #[test]
    fn client_request_financials() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::Financials)
                .is_ok()
        );
    }

    #[test]
    fn client_request_list() {
        assert!(
            CLIENT
                .stocks_request(
                    "market",
                    StocksEndpoint::List {
                        param: ListParam::Gainers
                    }
                )
                .is_ok()
        );
    }

    #[test]
    fn client_request_logo() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Logo).is_ok());
    }

    #[test]
    fn client_request_news() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::News { range: None })
                .is_ok()
        );
    }

    #[test]
    fn client_request_ohlc() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Ohlc).is_ok());
    }

    #[test]
    fn client_request_peers() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Peers).is_ok());
    }

    #[test]
    fn client_request_previous() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::Previous)
                .is_ok()
        );
    }

    #[test]
    fn client_request_price() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Price).is_ok());
    }

    #[test]
    fn client_request_quote() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Quote).is_ok());
    }

    #[test]
    fn client_request_relevant() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::Relevant)
                .is_ok()
        );
    }

    #[test]
    fn client_request_splits() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::Splits { duration })
                .is_ok()
        );
    }

    #[test]
    fn client_request_stats() {
        assert!(CLIENT.stocks_request(symbol, StocksEndpoint::Stats).is_ok());
    }

    #[test]
    fn client_request_threshold_securities() {
        assert!(
            CLIENT
                .stocks_request("market", StocksEndpoint::ThresholdSecurities { date: None })
                .is_ok()
        );
    }

    #[test]
    fn client_request_volume_by_venue() {
        assert!(
            CLIENT
                .stocks_request(symbol, StocksEndpoint::VolumeByVenue)
                .is_ok()
        );
    }
}
