extern crate iex;
extern crate serde_json;
use iex::types::Book;
use iex::{Client, Request};

fn main() {
    let client = Client::new();
    let resp = client.request(&Request::Book { symbol: "aapl" }).unwrap();
    let book = resp.try_into::<Book>();
    println!("{:?}", &book);
}
