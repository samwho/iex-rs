extern crate iex;
extern crate serde_json;
use iex::Book;
use iex::{Client, StocksEndpoint};

fn main() {
    let client = Client::new();
    let resp = client.stocks_request("aapl", StocksEndpoint::Book).unwrap();
    let book = resp.try_into::<Book>();
    println!("{:?}", &book);
}
