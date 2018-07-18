#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate iex;

use chrono::{TimeZone, Utc};
use iex::iextp::deep::{Message, PriceLevelUpdate, SecurityEvent};
use iex::iextp::Unmarshal;

lazy_static! {
    static ref ZIEXT: Vec<char> = ['Z', 'I', 'E', 'X', 'T'].to_vec();
}

enum MessageType {
    // Administrative message formats.
    SecurityEvent = 0x45,
    PriceLevelUpdateBuySide = 0x38,
    PriceLevelUpdateSellSide = 0x35,
}

#[test]
fn security_event() {
    let data = [
        // E = Security Event
        0x45,
        // O = Opening Process Complete
        0x4f,
        // 2017-04-17 09:30:00
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
    ];

    let expected = SecurityEvent {
        message_type: MessageType::SecurityEvent as u8,
        security_event: 0x4f,
        timestamp: Utc.ymd(2017, 04, 17).and_hms(9, 30, 0),
        symbol: ZIEXT.to_vec(),
    };

    match Message::unmarshal(&data) {
        Message::SecurityEvent(event) => assert_eq!(event, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn price_level_update_buy_side() {
    let data = [
        // Price level update on the Buy Side
        0x38,
        // Event processing complete
        0x01,
        // 2016-08-23 19:30:32.572715948
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
    ];

    let expected = PriceLevelUpdate {
        message_type: MessageType::PriceLevelUpdateBuySide as u8,
        event_flags: 1,
        timestamp: Utc.ymd(2016, 8, 23).and_hms_nano(19, 30, 32, 572715948),
        symbol: ZIEXT.to_vec(),
        size: 9700,
        price: 99.05,
    };

    match Message::unmarshal(&data) {
        Message::PriceLevelUpdate(update) => assert_eq!(update, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}

#[test]
fn price_level_update_sell_side() {
    let data = [
        // Price level update on the Sell Side
        0x35,
        // Event processing complete
        0x01,
        // 2016-08-23 19:30:32.572715948
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
    ];

    let expected = PriceLevelUpdate {
        message_type: MessageType::PriceLevelUpdateSellSide as u8,
        event_flags: 1,
        timestamp: Utc.ymd(2016, 8, 23).and_hms_nano(19, 30, 32, 572715948),
        symbol: ZIEXT.to_vec(),
        size: 9700,
        price: 99.05,
    };

    match Message::unmarshal(&data) {
        Message::PriceLevelUpdate(update) => assert_eq!(update, expected),
        got @ _ => panic!("\nexpected: {:?}, got: {:?}", expected, got),
    };
}
