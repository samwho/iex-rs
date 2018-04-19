#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate failure;

mod types;

use failure::Error;
use std::result;
use std::time::Duration;
use types::*;

pub type Result<T> = result::Result<T, Error>;

pub struct IexClient {
    http: reqwest::Client,
}

impl IexClient {
    pub fn new() -> Result<Self> {
        Ok(IexClient { 
            http: reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(10))
            .build()?
        })
    }

    pub fn book(&self, symbol: &str) -> Result<Book> {
        self.get(&format!("/stock/{}/book", symbol))
    }

    pub fn company(&self, symbol: &str) -> Result<Company> {
        self.get(&format!("/stock/{}/company", symbol))
    }

    pub fn delayed_quote(&self, symbol: &str) -> Result<DelayedQuote> {
        self.get(&format!("/stock/{}/delayed-quote", symbol))
    }

    pub fn dividends(&self, symbol: &str, duration: &str) -> Result<Vec<Dividend>> {
        self.get(&format!("/stock/{}/dividends/{}", symbol, duration))
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

    pub fn iex_regulation_sho_threshold_securities_list(&self, date: Option<&str>) -> Result<Vec<IEXRegulationSHOThresholdSecurity>> {
        self.get(&format!("/stock/market/threshold-securities/{}", date.unwrap_or("")))
    }

    pub fn iex_short_interest_list(&self, symbol: Option<&str>, date: Option<&str>) -> Result<Vec<IEXShortInterest>> {
        self.get(&format!("/stock/{}/short-interest/{}", symbol.unwrap_or("market"), date.unwrap_or("")))
    }

    pub fn stats(&self, symbol: &str) -> Result<Stats> {
        self.get(&format!("/stock/{}/stats", symbol))
    }

    pub fn list(&self, list: &str) -> Result<Vec<Quote>> {
        self.get(&format!("/stock/market/list/{}", list))
    }

    fn get<T>(&self, path: &str) -> Result<T> 
    where
        T: serde::de::DeserializeOwned
    {
        let uri = format!("{}{}", "https://api.iextrading.com/1.0", path);
        let res = self.http.get(&uri).send()?.error_for_status()?;
        Ok(serde_json::from_reader(res)?)
    }
}
