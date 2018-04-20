extern crate iex;
use iex::*;

fn main() {
    let iex = IexClient::new().unwrap();
    println!(
        "{:?}",
        iex.chart("aapl", Duration::YearToDate).unwrap()
    );
}
