#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate failure;

mod types;
use types::*;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub struct IexClient {
    http: reqwest::Client,
}

impl IexClient {
    pub fn new() -> Result<Self> {
        let http = reqwest::Client::builder()
            .gzip(true)
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(IexClient { http })
    }

    pub fn book(&self, symbol: &str) -> Result<Book> {
        Ok(self.get(&format!("/stock/{}/book", symbol))?)
    }

    pub fn company(&self, symbol: &str) -> Result<Company> {
        Ok(self.get(&format!("/stock/{}/company", symbol))?)
    }

    pub fn delayed_quote(&self, symbol: &str) -> Result<DelayedQuote> {
        Ok(self.get(&format!("/stock/{}/delayed-quote", symbol))?)
    }

    pub fn dividends(&self, symbol: &str, duration: &str) -> Result<Vec<Dividend>> {
        Ok(self.get(&format!("/stock/{}/dividends/{}", symbol, duration))?)
    }

    pub fn earnings(&self, symbol: &str) -> Result<Earnings> {
        Ok(self.get(&format!("/stock/{}/earnings", symbol))?)
    }

    pub fn effective_spread(&self, symbol: &str) -> Result<Vec<EffectiveSpread>> {
        Ok(self.get(&format!("/stock/{}/effective-spread", symbol))?)
    }

    pub fn financials(&self, symbol: &str) -> Result<Financials> {
        Ok(self.get(&format!("/stock/{}/financials", symbol))?)
    }

    fn get<T>(&self, path: &str) -> Result<T> 
    where
        T: serde::de::DeserializeOwned
    {
        let uri = format!("{}{}", "https://api.iextrading.com/1.0", path);
        Ok(serde_json::from_reader(self.http.get(&uri).send()?)?)
    }
}
