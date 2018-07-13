use chrono::{DateTime, Utc};
use iextp::Unmarshal;

#[derive(Debug, PartialEq)]
pub struct SystemEvent {
    pub message_type: u8,
    pub system_event: u8,
    pub timestamp: DateTime<Utc>,
}
impl Unmarshal for SystemEvent {
    fn unmarshal(buf: &[u8]) -> Self {
        // if buf.iter().count() < 10 {
        //     return None;
        // }

        SystemEvent {
            message_type: buf[0],
            system_event: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct SecurityDirectory {
    pub message_type: u8,
    // See Appendix A for flag values.
    pub flags: u8,
    // The time of the update event as set by the IEX Trading System logic.
    pub timestamp: DateTime<Utc>,
    // IEX-listed security represented in Nasdaq Integrated symbology.
    pub symbol: Vec<char>,
    // The number of shares that represent a round lot for the security.
    pub round_lot_size: u32,
    // The corporate action adjusted previous official closing price for
    // the security (e.g. stock split, dividend, rights offering).
    // When no corporate action has occurred, the Adjusted POC price
    // will be populated with the previous official close price. For
    // new issues (e.g., an IPO), this field will be the issue price.
    pub adjusted_POC_price: f64,
    // Indicates which Limit Up-Limit Down price band calculation
    // parameter is to be used.
    pub LULD_tier: u8,
}
impl Unmarshal for SecurityDirectory {
    fn unmarshal(buf: &[u8]) -> Self {
        // if buf.iter().count() < 31 {
        //     return None;
        // }

        SecurityDirectory {
            message_type: buf[0],
            flags: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            round_lot_size: Self::parse_u32(&buf[18..22]),
            adjusted_POC_price: Self::parse_price(&buf[22..30]),
            LULD_tier: buf[30],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TradingStatus {
    pub message_type: u8,
    // Trading status.
    pub trading_status: u8,
    // The time of the update event as set by the IEX Trading System logic.
    pub timestamp: DateTime<Utc>,
    // Security represented in Nasdaq integrated symbology.
    pub symbol: Vec<char>,
    // IEX populates the Reason field for IEX-listed securities when the
    // TradingStatus is TradingHalted or OrderAcceptancePeriod.
    // For non-IEX listed securities, the Reason field will be set to
    // ReasonNotAvailable when the trading status is TradingHalt.
    // The Reason will be blank when the trading status is TradingPause
    // or Trading.
    pub reason: Vec<char>,
}

impl Unmarshal for TradingStatus {
    fn unmarshal(buf: &[u8]) -> Self {
        // len buffer needs to be > 22
        TradingStatus {
            message_type: buf[0],
            trading_status: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            reason: Self::parse_string(&buf[18..22]),
        }
    }
}

// TODO: Implement TradingStatusType & TradingStatusReason enums.

#[derive(Debug, PartialEq)]
pub struct OperationalHaltStatus {
    pub message_type: u8,
    // Operational halt status identifier
    pub operational_halt_status: u8,
    // The time of the update event as set by the IEX Trading System logic.
    pub timestamp: DateTime<Utc>,
    // Security represented in Nasdaq integrated symbology.
    pub symbol: Vec<char>,
}

impl Unmarshal for OperationalHaltStatus {
    fn unmarshal(buf: &[u8]) -> Self {
        // len buffer needs to be > 18
        OperationalHaltStatus {
            message_type: buf[0],
            operational_halt_status: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ShortSalePriceTestStatus {
    pub message_type: u8,
    // Whether or not the ShortSalepriceTest is in effect.
    pub short_sale_price_test_status: bool,
    // The time of the update as set by the IEX Trading System logic.
    pub timestamp: DateTime<Utc>,
    // Security represented in Nasdaq integrated symbology.
    pub symbol: Vec<char>,
    // IEX populates the Detail field for IEX-listed securities;
    // this field will be set to DetailNotAvailable for non-IEX-listed
    // securities.
    pub detail: u8,
}
impl Unmarshal for ShortSalePriceTestStatus {
    fn unmarshal(buf: &[u8]) -> Self {
        // buffer len needs to be < 18
        ShortSalePriceTestStatus {
            message_type: buf[0],
            short_sale_price_test_status: buf[0] != 0x00,
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            detail: buf[18],
        }
    }
}

#[derive(Debug, PartialEq)]
// Trading message formats.
// TODO|REVIEW: potentially implement default trait to test `zero-sized`
// QuoteUpdate.
pub struct QuoteUpdate {
    pub message_type: u8,
    pub flags: u8,
    // The time an event triggered the quote update as set by the IEX Trading
    // System logic.
    pub timestamp: DateTime<Utc>,
    // Quoted symbol representation in Nasdaq integrated symbology.
    pub symbol: Vec<char>,
    // size of the quote at the bid, in number of shares.
    pub bid_size: u32,
    // price of the quote at the bid.
    pub bid_price: f64,
    // price of the quote at the ask.
    pub ask_price: f64,
    // size of the quote at the ask, in number of shares.
    pub ask_size: u32,
}
impl Unmarshal for QuoteUpdate {
    fn unmarshal(buf: &[u8]) -> Self {
        // len buffer < 42
        QuoteUpdate {
            message_type: buf[0],
            flags: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            bid_size: Self::parse_u32(&buf[18..22]),
            bid_price: Self::parse_price(&buf[22..30]),
            ask_price: Self::parse_price(&buf[30..38]),
            ask_size: Self::parse_u32(&buf[38..42]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TradeReport {
    pub message_type: u8,
    pub sale_condition_flags: u8,
    // The time an event triggered the trade (i.e., execution) as set
    // by the IEX Trading System logic.
    pub timestamp: DateTime<Utc>,
    // Traded symbol represented in Nasdaq integrated symbology.
    pub symbol: Vec<char>,
    // size of the trade, in number of shares.
    pub size: u32,
    // Execution price.
    pub price: f64,
    // IEX generated trade identifier. A given trade is uniquely
    // identified within a day by its trade_id.
    pub trade_id: i64,
}

impl Unmarshal for TradeReport {
    fn unmarshal(buf: &[u8]) -> Self {
        // len buffer < 42
        TradeReport {
            message_type: buf[0],
            sale_condition_flags: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            size: Self::parse_u32(&buf[18..22]),
            price: Self::parse_price(&buf[22..30]),
            trade_id: Self::parse_u64(&buf[30..38]) as i64,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OfficialPrice {
    pub message_type: u8,
    // price type identifier (Openingprice or Closingprice).
    pub price_type: u8,
    // The time an event triggered the official price calculation
    // (e.g., auction match) as set by the IEX Trading System logic.
    pub timestamp: DateTime<Utc>,
    // Security represented in Nasdaq Integrated symbology.
    pub symbol: Vec<char>,
    // IEX Official Opening or Closing price of an IEX-listed security.
    pub official_price: f64,
}

impl Unmarshal for OfficialPrice {
    fn unmarshal(buf: &[u8]) -> Self {
        // len buffer < 26
        OfficialPrice {
            message_type: buf[0],
            price_type: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            official_price: Self::parse_price(&buf[18..26]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TradeBreak {
    pub message_type: u8,
    pub sale_condition_flags: u8,
    // The time an event triggered the trade (i.e., execution) as set
    // by the IEX Trading System logic.
    pub timestamp: DateTime<Utc>,
    // Traded symbol represented in Nasdaq integrated symbology.
    pub symbol: Vec<char>,
    // size of the trade, in number of shares.
    pub size: u32,
    // Execution price.
    pub price: f64,
    // IEX generated trade identifier. A given trade is uniquely
    // identified within a day by its trade_id.
    pub trade_id: i64,
}

impl Unmarshal for TradeBreak {
    fn unmarshal(buf: &[u8]) -> Self {
        // len buffer < 42
        TradeBreak {
            message_type: buf[0],
            sale_condition_flags: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            size: Self::parse_u32(&buf[18..22]),
            price: Self::parse_price(&buf[22..30]),
            trade_id: Self::parse_u64(&buf[30..38]) as i64,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AuctionInformation {
    pub message_type: u8,
    pub auction_type: u8,
    // The time of the update event as set by the IEX Trading System logic.
    pub timestamp: DateTime<Utc>,
    // IEX-listed security represented in Nasdaq integrated symbology.
    pub symbol: Vec<char>,
    // Number of shares paried at the Reference price using orders on the
    // Auction Book.
    pub paired_shares: u32,
    // Clearing price at or within the Reference price Range using orders
    // on the Auction Book.
    pub reference_price: f64,
    // Clearing price using Eligible Auction Orders.
    pub indicative_clearing_price: f64,
    // Number of unpaired shares at the Reference price, using orders
    // on the Auction Book.
    pub imbalance_shares: u32,
    // Side of the imbalance.
    pub imbalance_side: u8,
    // Total number of automatic extensions an IPO, Halt, or Volatility
    // auction has received.
    pub extension_number: u8,
    // Projected time of the auction match.
    pub scheduled_auction_time: DateTime<Utc>,
    // Clearing price using orders on the Auction Book.
    pub auction_book_clearing_price: f64,
    // Reference price used for the auction collar, if any.
    pub collar_reference_price: f64,
    // Lower threshold price of the auction collar, if any.
    pub lower_auction_collar: f64,
    // Upper threshold price of the auction caller, if any.
    pub upper_auction_collar: f64,
}

impl Unmarshal for AuctionInformation {
    fn unmarshal(buf: &[u8]) -> Self {
        // len buffer < 80
        AuctionInformation {
            message_type: buf[0],
            auction_type: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            paired_shares: Self::parse_u32(&buf[18..22]),
            reference_price: Self::parse_price(&buf[22..30]),
            indicative_clearing_price: Self::parse_price(&buf[30..38]),
            imbalance_shares: Self::parse_u32(&buf[38..42]),
            imbalance_side: buf[42],
            extension_number: buf[43],
            scheduled_auction_time: Self::parse_event_time(&buf[44..48]),
            auction_book_clearing_price: Self::parse_price(&buf[48..56]),
            collar_reference_price: Self::parse_price(&buf[56..64]),
            lower_auction_collar: Self::parse_price(&buf[64..72]),
            upper_auction_collar: Self::parse_price(&buf[72..80]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SystemEventType {
    // Outside of heartbeat messages on the lower level protocol,
    // the start of day message is the first message in any trading session.
    StartOfMessages = 0x4f,
    // This message indicates that IEX is open and ready to start accepting
    // orders.
    StartOfSystemHours = 0x53,
    // This message indicates that DAY and GTX orders, as well as
    // market orders and pegged orders, are available for execution on IEX.
    StartOfRegularMarketHours = 0x52,
    // This message indicates that DAY orders, market orders, and pegged
    // orders are no longer accepted by IEX.
    EndOfRegularMarketHours = 0x4d,
    // This message indicates that IEX is now closed and will not accept
    // any new orders during this trading session. It is still possible to
    // receive messages after the end of day.
    EndOfSystemHours = 0x45,
    // This is always the last message sent in any trading session.
    EndOfMessages = 0x43,
}

pub enum LULD {
    Tier0 = 0x00,
    Tier1 = 0x1,
    Tier2 = 0x2,
}
