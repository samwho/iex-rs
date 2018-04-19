extern crate iex;
use iex::*;

fn main() {
    let iex = IexClient::new().unwrap();
    println!("{:?}", iex.iex_regulation_sho_threshold_securities_list(None).unwrap());
}
