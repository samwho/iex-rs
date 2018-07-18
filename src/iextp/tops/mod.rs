mod types;

pub use self::types::*;
use iextp::Unmarshal;

const CHANNEL_ID: u32 = 1;
const FEED_NAME: &str = "TOPS";

enum MessageProtocol {
    V1_5 = 0x8002,
    V1_6 = 0x8003,
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

    // Trading message formats.
    QuoteUpdate(QuoteUpdate),
    TradeReport(TradeReport),
    TradeBreak(TradeBreak),
    OfficialPrice(OfficialPrice),

    // Auction message formats.
    AuctionInformation(AuctionInformation),

    Unsupported(Vec<u8>),
}

impl Unmarshal for Message {
    fn unmarshal(buf: &[u8]) -> Self {
        match buf[0] {
            0x53 => Message::SystemEvent(SystemEvent::unmarshal(&buf)),
            0x44 => Message::SecurityDirectory(SecurityDirectory::unmarshal(&buf)),
            0x48 => Message::TradingStatus(TradingStatus::unmarshal(&buf)),
            0x4f => Message::OperationalHaltStatus(OperationalHaltStatus::unmarshal(&buf)),
            0x50 => Message::ShortSalePriceTestStatus(ShortSalePriceTestStatus::unmarshal(&buf)),
            0x51 => Message::QuoteUpdate(QuoteUpdate::unmarshal(&buf)),
            0x54 => Message::TradeReport(TradeReport::unmarshal(&buf)),
            0x42 => Message::TradeBreak(TradeBreak::unmarshal(&buf)),
            0x58 => Message::OfficialPrice(OfficialPrice::unmarshal(&buf)),
            0x41 => Message::AuctionInformation(AuctionInformation::unmarshal(&buf)),
            _ => Message::Unsupported(buf[..].to_vec()),
        }
    }
}
