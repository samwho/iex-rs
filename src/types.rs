#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
  pub symbol: String,
  pub company_name: String,
  pub primary_exchange: String,
  pub sector: String,
  pub calculation_price: String,
  pub open: f64,
  pub open_time: f64,
  pub close: f64,
  pub close_time: f64,
  pub high: f64,
  pub low: f64,
  pub latest_price: f64,
  pub latest_source: String,
  pub latest_time: String,
  pub latest_update: f64,
  pub latest_volume: f64,
  pub iex_realtime_price: f64,
  pub iex_realtime_size: f64,
  pub iex_last_updated: f64,
  pub delayed_price: f64,
  pub delayed_price_time: f64,
  pub previous_close: f64,
  pub change: f64,
  pub change_percent: f64,
  pub iex_market_percent: f64,
  pub iex_volume: f64,
  pub avg_total_volume: f64,
  pub iex_bid_price: f64,
  pub iex_bid_size: f64,
  pub iex_ask_price: f64,
  pub iex_ask_size: f64,
  pub market_cap: f64,
  pub pe_ratio: f64,
  pub week52_high: f64,
  pub week52_low: f64,
  pub ytd_change: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bid {
  pub price: f64,
  pub size: f64,
  pub timestamp: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ask {
  pub price: f64,
  pub size: f64,
  pub timestamp: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
  pub price: f64,
  pub size: f64,
  pub trade_id: f64,
  #[serde(rename = "isISO")] pub is_iso: bool,
  pub is_odd_lot: bool,
  pub is_outside_regular_hours: bool,
  pub is_single_price_cross: bool,
  pub is_trade_through_exempt: bool,
  pub timestamp: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemEvent {
  pub system_event: String,
  pub timestamp: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
  pub quote: Quote,
  pub bids: Vec<Bid>,
  pub asks: Vec<Ask>,
  pub trades: Vec<Trade>,
  pub system_event: SystemEvent,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
  pub symbol: String,
  pub company_name: String,
  pub exchange: String,
  pub industry: String,
  pub website: String,
  pub description: String,
  #[serde(rename = "CEO")] pub ceo: String,
  pub issue_type: String,
  pub sector: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct DelayedQuote {
  pub symbol: String,
  pub delayed_price: f64,
  pub high: f64,
  pub low: f64,
  pub delayed_size: f64,
  pub delayed_price_time: f64,
  pub processed_time: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Dividend {
  pub ex_date: String,
  pub payment_date: String,
  pub record_date: String,
  pub declared_date: String,
  pub amount: f64,
  pub flag: String,
  #[serde(rename = "type")] pub dtype: String, // because "type" is a keyword
  pub qualified: String,
  pub indicated: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Earning {
  #[serde(rename = "actualEPS")] pub actual_eps: f64,
  #[serde(rename = "consensusEPS")] pub consensus_eps: f64,
  #[serde(rename = "estimatedEPS")] pub estimated_eps: f64,
  pub announce_time: String,
  pub number_of_estimates: f64,
  #[serde(rename = "EPSSurpriseDollar")] pub eps_surprise_dollar: f64,
  #[serde(rename = "EPSReportDate")] pub eps_report_date: String,
  pub fiscal_period: String,
  pub fiscal_end_date: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Earnings {
  pub symbol: String,
  pub earnings: Vec<Earning>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct EffectiveSpread {
  pub volume: f64,
  pub venue: String,
  pub venue_name: String,
  pub effective_spread: f64,
  pub effective_quoted: f64,
  pub price_improvement: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Financial {
  pub report_date: String,
  pub gross_profit: f64,
  pub cost_of_revenue: f64,
  pub operating_revenue: f64,
  pub total_revenue: f64,
  pub operating_income: f64,
  pub net_income: f64,
  pub research_and_development: f64,
  pub operating_expense: f64,
  pub current_assets: f64,
  pub total_assets: f64,
  pub total_liabilities: f64,
  pub current_cash: f64,
  pub current_debt: f64,
  pub total_cash: f64,
  pub total_debt: f64,
  pub shareholder_equity: f64,
  pub cash_change: f64,
  pub cash_flow: f64,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Financials {
  pub symbol: String,
  pub financials: Vec<Financial>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct IEXRegulationSHOThresholdSecurity {
  pub trade_date: String,
  #[serde(rename = "SymbolinINETSymbology")] pub symbolin_inet_symbology: String,
  #[serde(rename = "SymbolInCQSSymbology")] pub symbolin_cqs_symbology: String,
  #[serde(rename = "SymbolinCMSSymbology")] pub symbolin_cms_symbology: String,
  pub security_name: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct IEXShortInterest {
  pub settlement_date: String,
  #[serde(rename = "SymbolinINETSymbology")] pub symbolin_inet_symbology: String,
  #[serde(rename = "SymbolinCQSSymbology")] pub symbolin_cqs_symbology: String,
  #[serde(rename = "SymbolinCMSSymbology")] pub symbolin_cms_symbology: String,
  pub security_name: String,
  pub company_name: String,
  pub current_short_interest: String,
  pub previous_short_interest: String,
  pub percent_change: String,
  pub average_daily_volume: String,
  pub daysto_cover: String,
  pub stock_adjustment_flag: String,
  pub new_issue_flag: String,
  pub revision_flag: String,
}
