#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

mod types;

use std::time::Duration;
use types::*;

const IEX_BASE_URL: &str = "https://api.iextrading.com/1.0";

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
        let path = format!("/stock/{}/book", symbol);
        Ok(self.get(&path)?)
    }

    fn get<T>(&self, path: &str) -> Result<T> 
    where
        T: serde::de::DeserializeOwned
    {
        let uri = format!("{}{}", IEX_BASE_URL, path);
        let res = self.http.get(&uri).send()?;
        Ok(serde_json::from_reader(res)?)
    }
}
