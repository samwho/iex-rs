#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

mod types;

use std::time::Duration;
use types::*;


pub type Error = Box<std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct IexClient {
    http: reqwest::Client,
}

impl IexClient {
    pub fn new() -> Result<Self> {
        let http = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(IexClient { http })
    }

    pub fn book(&self, symbol: &str) -> Result<BookResponse> {
        Ok(self.get(&format!("/stock/{}/book", symbol))?)
    }

    pub fn company(&self, symbol: &str) -> Result<CompanyResponse> {
        Ok(self.get(&format!("/stock/{}/company", symbol))?)
    }

    pub fn delayed_quote(&self, symbol: &str) -> Result<DelayedQuoteResponse> {
        Ok(self.get(&format!("/stock/{}/delayed-quote", symbol))?)
    }

    fn get<T>(&self, path: &str) -> Result<T> 
    where
        T: serde::de::DeserializeOwned
    {
        let uri = format!("{}{}", "https://api.iextrading.com/1.0", path);
        Ok(serde_json::from_reader(self.http.get(&uri).send()?)?)
    }
}
