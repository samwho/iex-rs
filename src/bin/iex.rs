extern crate iex;
use iex::*;

fn main() {
    let iex = IexClient::new().unwrap();
    println!("{:?}", iex.iex_short_interest_list(None, Some("sample")).unwrap());
}
