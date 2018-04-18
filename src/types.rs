#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
  symbol: String,
  company_name: String,
  primary_exchange: String,
  sector: String,
  calculation_price: String,
  open: f64,
  open_time: f64,
  close: f64,
  close_time: f64,
  high: f64,
  low: f64,
  latest_price: f64,
  latest_source: String,
  latest_time: String,
  latest_update: f64,
  latest_volume: f64,
  iex_realtime_price: f64,
  iex_realtime_size: f64,
  iex_last_updated: f64,
  delayed_price: f64,
  delayed_price_time: f64,
  previous_close: f64,
  change: f64,
  change_percent: f64,
  iex_market_percent: f64,
  iex_volume: f64,
  avg_total_volume: f64,
  iex_bid_price: f64,
  iex_bid_size: f64,
  iex_ask_price: f64,
  iex_ask_size: f64,
  market_cap: f64,
  pe_ratio: f64,
  week52_high: f64,
  week52_low: f64,
  ytd_change: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bid {
  price: f64,
  size: f64,
  timestamp: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ask {
  price: f64,
  size: f64,
  timestamp: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
  price: f64,
  size: f64,
  trade_id: f64,
  #[serde(rename = "isISO")] is_iso: bool,
  is_odd_lot: bool,
  is_outside_regular_hours: bool,
  is_single_price_cross: bool,
  is_trade_through_exempt: bool,
  timestamp: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemEvent {
  system_event: String,
  timestamp: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
  quote: Quote,
  bids: Vec<Bid>,
  asks: Vec<Ask>,
  trades: Vec<Trade>,
  system_event: SystemEvent,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
  symbol: String,
  company_name: String,
  exchange: String,
  industry: String,
  website: String,
  description: String,
  #[serde(rename = "CEO")] ceo: String,
  issue_type: String,
  sector: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct DelayedQuote {
  symbol: String,
  delayed_price: f64,
  high: f64,
  low: f64,
  delayed_size: f64,
  delayed_price_time: f64,
  processed_time: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Dividend {
  ex_date: String,
  payment_date: String,
  record_date: String,
  declared_date: String,
  amount: f64,
  flag: String,
  #[serde(rename = "type")] dtype: String, // because "type" is a keyword
  qualified: String,
  indicated: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Earning {
  #[serde(rename = "actualEPS")] actual_eps: f64,
  #[serde(rename = "consensusEPS")] consensus_eps: f64,
  #[serde(rename = "estimatedEPS")] estimated_eps: f64,
  announce_time: String,
  number_of_estimates: f64,
  #[serde(rename = "EPSSurpriseDollar")] eps_surprise_dollar: f64,
  #[serde(rename = "EPSReportDate")] eps_report_date: String,
  fiscal_period: String,
  fiscal_end_date: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Earnings {
  symbol: String,
  earnings: Vec<Earning>,
}
