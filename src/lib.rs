#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod types;

use failure::Error;
use std::result;
use std::time;
use types::*;

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
    fn default() -> Duration { Duration::OneMonth }
}

pub struct IexClient {
    http: reqwest::Client,
}

impl IexClient {
    pub fn new() -> Result<Self> {
        Ok(IexClient {
            http: reqwest::Client::builder()
                .gzip(true)
                .timeout(time::Duration::from_secs(10))
                .build()?,
        })
    }

    pub fn book(&self, symbol: &str) -> Result<Book> {
        self.get(&format!("/stock/{}/book", symbol))
    }

    pub fn chart(&self, symbol: &str, duration: Duration) -> Result<Vec<ChartDataPoint>> {
        self.get(&format!(
            "/stock/{}/chart/{}",
            symbol,
            duration.to_string()
        ))
    }

    pub fn company(&self, symbol: &str) -> Result<Company> {
        self.get(&format!("/stock/{}/company", symbol))
    }

    pub fn delayed_quote(&self, symbol: &str) -> Result<DelayedQuote> {
        self.get(&format!("/stock/{}/delayed-quote", symbol))
    }

    pub fn dividends(&self, symbol: &str, duration: Duration) -> Result<Vec<Dividend>> {
        self.get(&format!("/stock/{}/dividends/{}", symbol, duration.to_string()))
    }

    pub fn earnings(&self, symbol: &str) -> Result<Earnings> {
        self.get(&format!("/stock/{}/earnings", symbol))
    }

    pub fn effective_spread(&self, symbol: &str) -> Result<Vec<EffectiveSpread>> {
        self.get(&format!("/stock/{}/effective-spread", symbol))
    }

    pub fn financials(&self, symbol: &str) -> Result<Financials> {
        self.get(&format!("/stock/{}/financials", symbol))
    }

    pub fn iex_regulation_sho_threshold_securities_list(
        &self,
        date: Option<&str>,
    ) -> Result<Vec<IEXRegulationSHOThresholdSecurity>> {
        self.get(&format!(
            "/stock/market/threshold-securities/{}",
            date.unwrap_or("")
        ))
    }

    pub fn iex_short_interest_list(
        &self,
        symbol: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<IEXShortInterest>> {
        self.get(&format!(
            "/stock/{}/short-interest/{}",
            symbol.unwrap_or("market"),
            date.unwrap_or("")
        ))
    }

    pub fn stats(&self, symbol: &str) -> Result<Stats> {
        self.get(&format!("/stock/{}/stats", symbol))
    }

    pub fn list(&self, list: &str) -> Result<Vec<Quote>> {
        self.get(&format!("/stock/market/list/{}", list))
    }

    pub fn logo(&self, symbol: &str) -> Result<Logo> {
        self.get(&format!("/stock/{}/logo", symbol))
    }

    pub fn news(&self, symbol: &str) -> Result<Vec<News>> {
        // TODO: this also takes a count argument, implement it.
        self.get(&format!("/stock/{}/news", symbol))
    }

    pub fn ohlc(&self, symbol: &str) -> Result<OHLC> {
        self.get(&format!("/stock/{}/ohlc", symbol))
    }

    pub fn peers(&self, symbol: &str) -> Result<Vec<String>> {
        self.get(&format!("/stock/{}/peers", symbol))
    }

    pub fn previous(&self, symbol: &str) -> Result<Previous> {
        // TODO: It's possible to pass in "market" as an argument here
        // and get one entry for each symbol. We need to handle that
        // scenario.
        self.get(&format!("/stock/{}/previous", symbol))
    }

    pub fn price(&self, symbol: &str) -> Result<f64> {
        self.get(&format!("/stock/{}/price", symbol))
    }

    pub fn quote(&self, symbol: &str) -> Result<Quote> {
        self.get(&format!("/stock/{}/quote", symbol))
    }

    pub fn relevant(&self, symbol: &str) -> Result<Relevant> {
        self.get(&format!("/stock/{}/relevant", symbol))
    }

    pub fn splits(&self, symbol: &str, duration: Option<&str>) -> Result<Vec<Split>> {
        self.get(&format!(
            "/stock/{}/splits/{}",
            symbol,
            duration.unwrap_or("")
        ))
    }

    pub fn time_series(&self, symbol: &str, duration: Duration) -> Result<Vec<ChartDataPoint>> {
        self.chart(symbol, duration)
    }

    pub fn volume_by_venue(&self, symbol: &str) -> Result<Vec<VolumeByVenue>> {
        self.get(&format!("/stock/{}/volume-by-venue", symbol))
    }

    pub fn symbols(&self) -> Result<Vec<Symbol>> {
        self.get("/ref-data/symbols")
    }

    fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let uri = format!("{}{}", "https://api.iextrading.com/1.0", path);
        println!("{}", uri);
        let res = self.http.get(&uri).send()?.error_for_status()?;
        Ok(serde_json::from_reader(res)?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn book() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.book("aapl").is_ok());
    }

    #[test]
    fn chart() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.chart("aapl", ::Duration::default()).is_ok());
    }

    #[test]
    fn company() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.company("aapl").is_ok());
    }

    #[test]
    fn delayed_quote() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.delayed_quote("aapl").is_ok());
    }

    #[test]
    fn dividends() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.dividends("aapl", ::Duration::default()).is_ok());
    }

    #[test]
    fn earnings() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.earnings("aapl").is_ok());
    }

    #[test]
    fn effective_spread() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.effective_spread("aapl").is_ok());
    }

    #[test]
    fn financials() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.financials("aapl").is_ok());
    }

    #[test]
    fn iex_regulation_sho_threshold_securities_list() {
        let iex = ::IexClient::new().unwrap();
        assert!(
            iex.iex_regulation_sho_threshold_securities_list(Some("sample"))
                .is_ok()
        );
    }

    #[test]
    fn iex_short_interest_list() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.iex_short_interest_list(None, None).is_ok());
    }

    #[test]
    fn stats() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.stats("aapl").is_ok());
    }

    #[test]
    fn list() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.list("gainers").is_ok());
    }

    #[test]
    fn logo() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.logo("aapl").is_ok());
    }

    #[test]
    fn news() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.news("aapl").is_ok());
    }

    #[test]
    fn ohlc() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.ohlc("aapl").is_ok());
    }

    #[test]
    fn peers() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.peers("aapl").is_ok());
    }

    #[test]
    fn previous() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.previous("aapl").is_ok());
    }

    #[test]
    fn price() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.price("aapl").is_ok());
    }

    #[test]
    fn quote() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.quote("aapl").is_ok());
    }

    #[test]
    fn relevant() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.relevant("aapl").is_ok());
    }

    #[test]
    fn splits() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.splits("aapl", None).is_ok());
    }

    #[test]
    fn time_series() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.time_series("aapl", ::Duration::default()).is_ok());
    }

    #[test]
    fn volume_by_venue() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.volume_by_venue("aapl").is_ok());
    }

    #[test]
    fn symbols() {
        let iex = ::IexClient::new().unwrap();
        assert!(iex.symbols().is_ok());
    }
}
