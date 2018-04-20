extern crate iex;
use iex::*;

fn main() {
    let iex = IexClient::new().unwrap();
    println!(
        "{:?}",
        iex.chart_with_params(
            "aapl",
            Duration::YearToDate,
            ChartParamsBuilder::default()
                .chart_simplify(true)
                .build()
                .unwrap()
        ).unwrap()
    );
}
