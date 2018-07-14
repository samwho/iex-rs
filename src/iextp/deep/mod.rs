use iextp::tops::{
    AuctionInformation, OfficialPrice, OperationalHaltStatus, SecurityDirectory,
    ShortSalePriceTestStatus, SystemEvent, TradeBreak, TradeReport, TradingStatus,
};
use iextp::Unmarshal;

mod types;

pub use self::types::*;

const CHANNEL_ID: u32 = 1;
const FEED_NAME: &str = "DEEP";

enum MessageProtocol {
    V1 = 0x8004,
}

/// A message is an atomic piece of application information carried by an IEX
/// -
/// TP segment.  The contents of a message are
/// defined by the message protocol in use.
#[derive(Debug, PartialEq)]
pub enum Message {
    // Administrative message formats
    SystemEvent(SystemEvent),
    SecurityDirectory(SecurityDirectory),
    TradingStatus(TradingStatus),
    OperationalHaltStatus(OperationalHaltStatus),
    ShortSalePriceTestStatus(ShortSalePriceTestStatus),
    SecurityEvent(SecurityEvent),
    PriceLevelUpdate(PriceLevelUpdate),

    // Trading message formats.
    TradeReport(TradeReport),
    OfficialPrice(OfficialPrice),
    TradeBreak(TradeBreak),

    // Auction message formats.
    AuctionInformation(AuctionInformation),

    Unsupported(Vec<u8>),
}

impl Unmarshal for Message {
    fn unmarshal(buf: &[u8]) -> Self {
        match buf[0] {
            // REVIEW: Should I make these literals consts?
            0x53 => Message::SystemEvent(SystemEvent::unmarshal(&buf)),
            0x44 => Message::SecurityDirectory(SecurityDirectory::unmarshal(&buf)),
            0x48 => Message::TradingStatus(TradingStatus::unmarshal(&buf)),
            0x4f => Message::OperationalHaltStatus(OperationalHaltStatus::unmarshal(&buf)),
            0x45 => Message::SecurityEvent(SecurityEvent::unmarshal(&buf)),
            0x38 | 0x35 => Message::PriceLevelUpdate(PriceLevelUpdate::unmarshal(&buf)),
            0x50 => Message::ShortSalePriceTestStatus(ShortSalePriceTestStatus::unmarshal(&buf)),
            0x54 => Message::TradeReport(TradeReport::unmarshal(&buf)),
            0x58 => Message::OfficialPrice(OfficialPrice::unmarshal(&buf)),
            0x42 => Message::TradeBreak(TradeBreak::unmarshal(&buf)),
            0x41 => Message::AuctionInformation(AuctionInformation::unmarshal(&buf)),
            _ => Message::Unsupported(buf[..].to_vec()),
        }
    }
}
