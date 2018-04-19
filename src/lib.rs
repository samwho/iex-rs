#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate failure;

mod types;
use types::*;
use std::time::Duration;

pub type Result<T> = std::result::Result<T, failure::Error>;

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

    fn get<T>(&self, path: &str) -> Result<T> 
    where
        T: serde::de::DeserializeOwned
    {
        let uri = format!("{}{}", "https://api.iextrading.com/1.0", path);
        Ok(serde_json::from_reader(self.http.get(&uri).send()?)?)
    }
}
