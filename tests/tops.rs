#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate iex;

use chrono::{TimeZone, Utc};
use iex::iextp::tops::{
    AuctionInformation, Message, OfficialPrice, OperationalHaltStatus, QuoteUpdate,
    SecurityDirectory, ShortSalePriceTestStatus, SystemEvent, SystemEventType, TradeBreak,
    TradeReport, TradingStatus, LULD,
};
use iex::iextp::Unmarshal;

lazy_static! {
    static ref ZIEXT: Vec<char> = ['Z', 'I', 'E', 'X', 'T'].to_vec();
}

enum MessageType {
    // Administrative message formats.
    SystemEvent = 0x53,
    SecurityDirectory = 0x44,
    TradingStatus = 0x48,
    OperationalHaltStatus = 0x4f,
    ShortSalePriceTestStatus = 0x50,

    // Trading message formats.
    QuoteUpdate = 0x51,
    TradeReport = 0x54,
    TradeBreak = 0x42,
    OfficialPrice = 0x58,

    // Auction message formats.
    AuctionInformation = 0x41,
}

#[test]
fn system_event() {
    let data = [
        // S = System Event
        0x53,
        // End of System Hours
        0x45,
        // 2017-04-17 17:00:00
        0x00,
        0xa0,
        0x99,
        0x97,
        0xe9,
        0x3d,
        0xb6,
        0x14,
    ];

    let expected = SystemEvent {
        message_type: MessageType::SystemEvent as u8,
        system_event: SystemEventType::EndOfSystemHours as u8,
        timestamp: Utc.ymd(2017, 04, 17).and_hms(17, 0, 0),
    };

    match Message::unmarshal(&data) {
        Message::SystemEvent(sys_event) => assert_eq!(sys_event, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn security_deposit() {
    let data = [
        // D = Security Directory
        0x44,
        // Test security, not an ETP, not a When Issued security
        0x80,
        // 2017-04-17 07:40:00
        0x00,
        0x20,
        0x89,
        0x7b,
        0x5a,
        0x1f,
        0xb6,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
        // 100 shares
        0x64,
        0x00,
        0x00,
        0x00,
        // $99.05
        0x24,
        0x1d,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // Tier 1 NMS Stock
        0x01,
    ];

    let expected = SecurityDirectory {
        message_type: MessageType::SecurityDirectory as u8,
        flags: 0x80,
        timestamp: Utc.ymd(2017, 4, 17).and_hms(7, 40, 0),
        symbol: vec!['Z', 'I', 'E', 'X', 'T'],
        round_lot_size: 100,
        adjusted_POC_price: 99.05,
        LULD_tier: LULD::Tier1 as u8,
    };

    match Message::unmarshal(&data) {
        Message::SecurityDirectory(sec_dir) => assert_eq!(sec_dir, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn trading_status() {
    let data = [
        // H = Trading Status
        0x48,
        // H = Trading Halted
        0x48,
        // 2016-08-23 15:30:32.572715948
        0xac,
        0x63,
        0xc0,
        0x20,
        0x96,
        0x86,
        0x6d,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
        // T1 = Halt News Pending
        0x54,
        0x31,
        0x20,
        0x20,
    ];

    let expected = TradingStatus {
        message_type: MessageType::TradingStatus as u8,
        trading_status: 0x48,
        timestamp: Utc.ymd(2016, 8, 23).and_hms_nano(19, 30, 32, 572715948),
        symbol: ZIEXT.to_vec(),
        reason: vec!['T', '1'],
    };

    match Message::unmarshal(&data) {
        Message::TradingStatus(trade_status) => assert_eq!(trade_status, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn operational_halt_status() {
    let data = [
        // O = Operational Halt Status
        0x4f,
        // O = Operationally halted on IEX
        0x4f,
        // 2016-08-23 15:30:32.572715948
        0xac,
        0x63,
        0xc0,
        0x20,
        0x96,
        0x86,
        0x6d,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
    ];

    let expected = OperationalHaltStatus {
        message_type: MessageType::OperationalHaltStatus as u8,
        operational_halt_status: 0x4f,
        timestamp: Utc.ymd(2016, 8, 23).and_hms_nano(19, 30, 32, 572715948),
        symbol: ZIEXT.to_vec(),
    };

    match Message::unmarshal(&data) {
        Message::OperationalHaltStatus(ohs) => assert_eq!(ohs, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn short_sale_price_test_status() {
    let data = [
        // P = Short Sale Price Test Status
        0x50,
        // Short Sale Price Test in effect
        0x01,
        // 2016-08-23 15:30:32.572715948
        0xac,
        0x63,
        0xc0,
        0x20,
        0x96,
        0x86,
        0x6d,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
        // Activated
        0x41,
    ];

    let expected = ShortSalePriceTestStatus {
        message_type: MessageType::ShortSalePriceTestStatus as u8,
        short_sale_price_test_status: true,
        timestamp: Utc.ymd(2016, 8, 23).and_hms_nano(19, 30, 32, 572715948),
        symbol: ZIEXT.to_vec(),
        detail: 0x41,
    };

    match Message::unmarshal(&data) {
        Message::ShortSalePriceTestStatus(sspts) => assert_eq!(sspts, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn quote_update() {
    let data = [
        // Q = Quote Update
        0x51,
        // Active and regular market session
        0x00,
        // 2016-08-23 15:30:32.572715948
        0xac,
        0x63,
        0xc0,
        0x20,
        0x96,
        0x86,
        0x6d,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
        // 9,700 shares
        0xe4,
        0x25,
        0x00,
        0x00,
        // $99.05
        0x24,
        0x1d,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // $99.07
        0xec,
        0x1d,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // 1,000 shares;
        0xe8,
        0x03,
        0x00,
        0x00,
    ];

    let expected = QuoteUpdate {
        message_type: MessageType::QuoteUpdate as u8,
        flags: 0,
        timestamp: Utc.ymd(2016, 8, 23).and_hms_nano(19, 30, 32, 572715948),
        symbol: ZIEXT.to_vec(),
        bid_size: 9700,
        bid_price: 99.05,
        ask_price: 99.07,
        ask_size: 1000,
    };

    match Message::unmarshal(&data) {
        Message::QuoteUpdate(quote) => assert_eq!(quote, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn trade_report() {
    let data = [
        0x54,
        0x00,
        // 2016-08-23 15:30:32.572715948
        0xac,
        0x63,
        0xc0,
        0x20,
        0x96,
        0x86,
        0x6d,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
        // 100 shares
        0x64,
        0x00,
        0x00,
        0x00,
        // $99.05
        0x24,
        0x1d,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // 429974
        0x96,
        0x8f,
        0x06,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];

    let expected = TradeReport {
        message_type: MessageType::TradeReport as u8,
        sale_condition_flags: 0,
        timestamp: Utc.ymd(2016, 8, 23).and_hms_nano(19, 30, 32, 572715948),
        symbol: ZIEXT.to_vec(),
        size: 100,
        price: 99.05,
        trade_id: 429974,
    };

    match Message::unmarshal(&data) {
        Message::TradeReport(tr) => assert_eq!(tr, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn official_price() {
    let data = [
        // X = Official Price
        0x58,
        // Q = IEX Official Opening Price
        0x51,
        // 2017-04-17 09:30:00.000000000
        0x00,
        0xf0,
        0x30,
        0x2a,
        0x5b,
        0x25,
        0xb6,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
        // $99.05
        0x24,
        0x1d,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];

    let expected = OfficialPrice {
        message_type: MessageType::OfficialPrice as u8,
        price_type: 0x51,
        timestamp: Utc.ymd(2017, 4, 17).and_hms(9, 30, 0),
        symbol: ZIEXT.to_vec(),
        official_price: 99.05,
    };

    match Message::unmarshal(&data) {
        Message::OfficialPrice(price) => assert_eq!(price, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn trade_break() {
    let data = [
        // B = Trade Break
        0x42,
        // Non-ISO, Regular Market Session, Round or mixed lot, subject to Rule 611.
        0x00,
        // 2016-08-23 15:32:04.912754610
        0xb2,
        0x8f,
        0xa5,
        0xa0,
        0xab,
        0x86,
        0x6d,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
        // 100 shares
        0x64,
        0x00,
        0x00,
        0x00,
        // $99.05
        0x24,
        0x1d,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // 429974
        0x96,
        0x8f,
        0x06,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];

    let expected = TradeBreak {
        message_type: MessageType::TradeBreak as u8,
        sale_condition_flags: 0,
        timestamp: Utc.ymd(2016, 8, 23).and_hms_nano(19, 32, 04, 912754610),
        symbol: ZIEXT.to_vec(),
        size: 100,
        price: 99.05,
        trade_id: 429974,
    };

    match Message::unmarshal(&data) {
        Message::TradeBreak(trade_break) => assert_eq!(trade_break, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn auction_information() {
    let data = [
        // A = Auction Information
        0x41,
        // C = Closing Auction
        0x43,
        // 2017-04-17 15:50:12.462929885
        0xdd,
        0xc7,
        0xf0,
        0x9a,
        0x1a,
        0x3a,
        0xb6,
        0x14,
        // ZIEXT
        0x5a,
        0x49,
        0x45,
        0x58,
        0x54,
        0x20,
        0x20,
        0x20,
        // NOTE: The spec example says 100,000 shares, but this is not correct.
        // It's actually a 27,160 shares as a little endian 4-byte integer.
        // 100,000 shares
        0x18,
        0x6a,
        0x00,
        0x00,
        // $99.05
        0x24,
        0x1d,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // $99.10
        0x18,
        0x1f,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // NOTE: The spec example says 10,000 shares, but this is not correct.
        // It's actually a 4,135 shares as a little endian 4-byte integer.
        // 10,000 shares
        0x27,
        0x10,
        0x00,
        0x00,
        // B = buy-side imbalance
        0x42,
        // 0 extensions
        0x00,
        // 2017-04-17 16:00:00
        0x80,
        0xe6,
        0xf4,
        0x58,
        // $99.15
        0x0c,
        0x21,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // $99.04
        0xc0,
        0x1c,
        0x0f,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // $89.13
        0xa4,
        0x99,
        0x0d,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        // $108.95
        0xdc,
        0x9f,
        0x10,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];

    let expected = AuctionInformation {
        message_type: MessageType::AuctionInformation as u8,
        auction_type: 0x43,
        timestamp: Utc.ymd(2017, 4, 17).and_hms_nano(15, 50, 12, 462929885),
        symbol: ZIEXT.to_vec(),
        paired_shares: 27160,
        reference_price: 99.05,
        indicative_clearing_price: 99.10,
        imbalance_shares: 4135,
        imbalance_side: 0x42,
        extension_number: 0,
        scheduled_auction_time: Utc.ymd(2017, 4, 17).and_hms(16, 0, 0),
        auction_book_clearing_price: 99.15,
        collar_reference_price: 99.04,
        lower_auction_collar: 89.13,
        upper_auction_collar: 108.95,
    };

    match Message::unmarshal(&data) {
        Message::AuctionInformation(auction_info) => assert_eq!(auction_info, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}
