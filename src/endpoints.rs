use Endpoint;

#[derive(PartialEq, Eq)]
/// The `StocksEndpoint` enum allows for HTTP requests matching to a IEX Stocks Endpoint API.
// TODO:(Request): Add documentation from IEX website.
// TODO: use display_percent
pub enum StocksEndpoint<'a> {
    Book,
    Chart {
        duration: Duration<'a>,
        params: Option<Vec<ChartParam>>,
    },
    Company,
    DelayedQuote,
    Dividends {
        duration: Duration<'a>,
    },
    Earnings,
    EffectiveSpread,
    Financials,
    List {
        param: ListParam,
    },
    Logo,
    News {
        range: Option<i32>,
    },
    Ohlc,
    Peers,
    Previous,
    Price,
    Quote,
    Relevant,
    Splits {
        duration: Duration<'a>,
    },
    Stats,

    // TODO: IEX Short Interest List
    /// IEX Regulation SHO Threshold Securities List
    TimeSeries,
    ThresholdSecurities {
        date: Option<Duration<'a>>,
    },
    // TODO(ShortInterest): implement variant.
    VolumeByVenue,
}

impl<'a> Endpoint for StocksEndpoint<'a> {
    fn to_endpoint(self) -> String {
        match self {
            StocksEndpoint::Book => String::from("book"),

            StocksEndpoint::Chart { duration, params } => format!(
                "chart/{}?{chart_params}",
                duration.to_string(),
                chart_params = match params {
                    Some(parameters) => {
                        let chart_params: String = parameters
                            .iter()
                            .map(|param| param.to_string() + "/")
                            .collect();
                        chart_params
                    }
                    None => String::from(""),
                }
            ),

            StocksEndpoint::Company => String::from("company"),

            StocksEndpoint::DelayedQuote => String::from("delayed-quote"),

            StocksEndpoint::Dividends { duration } => format!("dividends/{}", duration.to_string()),

            StocksEndpoint::Earnings => String::from("earnings"),

            StocksEndpoint::EffectiveSpread => String::from("effective-spread"),

            StocksEndpoint::Financials => String::from("financials"),

            StocksEndpoint::List { param } => format!("list/{}", param.to_string()),

            StocksEndpoint::Logo => String::from("logo"),

            StocksEndpoint::News { range } => format!(
                "news/last/{}",
                range.map(|r| r.to_string()).unwrap_or("".to_string()),
            ),

            StocksEndpoint::Ohlc => String::from("ohlc"),

            StocksEndpoint::Peers => String::from("peers"),

            StocksEndpoint::Previous => String::from("previous"),

            StocksEndpoint::Price => String::from("price"),

            StocksEndpoint::Quote => String::from("quote"),

            StocksEndpoint::Relevant => String::from("relevant"),

            StocksEndpoint::Stats => String::from("stats"),

            StocksEndpoint::Splits { duration } => format!("splits/{}", duration.to_string()),

            StocksEndpoint::TimeSeries => String::from("time-series"),

            StocksEndpoint::ThresholdSecurities { date } => format!(
                "threshold-securities/{}",
                date.unwrap_or(Duration::None).to_string(),
            ),

            StocksEndpoint::VolumeByVenue => String::from("volume-by-venue"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Duration<'a> {
    FiveYears,
    TwoYears,
    OneYear,
    YearToDate,
    SixMonths,
    ThreeMonths,
    OneMonth,
    OneDay,
    Date(&'a str),
    Dynamic,
    None,
}

impl<'a> ToString for Duration<'a> {
    fn to_string(&self) -> String {
        match self {
            Duration::FiveYears => String::from("5y"),
            Duration::TwoYears => String::from("2y"),
            Duration::OneYear => String::from("1y"),
            Duration::YearToDate => String::from("ytd"),
            Duration::SixMonths => String::from("6m"),
            Duration::ThreeMonths => String::from("3m"),
            Duration::OneMonth => String::from("1m"),
            Duration::OneDay => String::from("1d"),
            Duration::Date(date) => format!("date/{}", date),
            Duration::Dynamic => String::from("dynamic"),
            Duration::None => String::from(""),
        }
    }
}

impl<'a> Default for Duration<'a> {
    fn default() -> Duration<'a> {
        Duration::OneMonth
    }
}

#[derive(PartialEq, Eq)]
pub enum ListParam {
    MostActive,
    Gainers,
    Losers,
    IexVolume,
    IexPercent,
}

impl ToString for ListParam {
    fn to_string(&self) -> String {
        match self {
            ListParam::MostActive => String::from("mostactive"),
            ListParam::Gainers => String::from("gainers"),
            ListParam::Losers => String::from("losers"),
            ListParam::IexVolume => String::from("iexvolume"),
            ListParam::IexPercent => String::from("iexpercent"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ChartParam {
    /// If true, 1d chart will reset at midnight instead of the default behavior of 9:30am ET.
    Reset(bool),
    /// If true, runs a polyline simplification using the Douglas-Peucker algorithm. This is useful if plotting sparkline charts.
    Simplify(bool),
    /// If passed, chart data will return every Nth element as defined by `Interval`.
    Interval(usize),
    /// If true, changeOverTime and marketChangeOverTime will be relative to previous day close instead of the first value.
    ChangeFromClose(bool),
    /// If passed, chart data will return the last N elements.
    Last(usize),
}

impl ToString for ChartParam {
    fn to_string(&self) -> String {
        match self {
            ChartParam::Reset(res) => format!("chartReset={}", res),
            ChartParam::Simplify(res) => format!("chartSimplify={}", res),
            ChartParam::Interval(res) => format!("chartInterval={}", res),
            ChartParam::ChangeFromClose(res) => format!("changeFromClose={}", res),
            ChartParam::Last(res) => format!("chartLast={}", res),
        }
    }
}

// pub enum ReferenceEndpoint<'a> {
//     Symbols,
//     CorporateActions { date: Option<&'a str> },
// }

// impl<'a> ReferenceEndpoint<'a> {
//     pub fn to_endpoint(self) -> String {
//         match self {
//             ReferenceEndpoint::Symbols => String::from("symbols"),
//             ReferenceEndpoint::CorporateActions => format!("{}", )
//         }
//     }
// }
