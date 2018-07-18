use chrono::{DateTime, Utc};
use iextp::Unmarshal;

#[derive(Debug, PartialEq)]
pub struct SecurityEvent {
    pub message_type: u8,
    pub security_event: u8,
    pub timestamp: DateTime<Utc>,
    pub symbol: Vec<char>,
}
impl Unmarshal for SecurityEvent {
    fn unmarshal(buf: &[u8]) -> Self {
        SecurityEvent {
            message_type: buf[0],
            security_event: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
        }
    }
}

// REVIEW: Should I make these literals consts?
pub enum SecurityEventType {
    OpeningProcessComplete = 0x4f,
    ClosingProcessComplete = 0x43,
}

#[derive(Debug, PartialEq)]
pub struct PriceLevelUpdate {
    pub message_type: u8,
    pub event_flags: u8,
    pub timestamp: DateTime<Utc>,
    pub symbol: Vec<char>,
    pub size: u32,
    pub price: f64,
}
impl Unmarshal for PriceLevelUpdate {
    fn unmarshal(buf: &[u8]) -> Self {
        PriceLevelUpdate {
            message_type: buf[0],
            event_flags: buf[1],
            timestamp: Self::parse_timestamp(&buf[2..10]),
            symbol: Self::parse_string(&buf[10..18]),
            size: Self::parse_u32(&buf[18..22]),
            price: Self::parse_price(&buf[22..30]),
        }
    }
}

// REVIEW: Should I make these literals consts?
pub enum PriceLevelUpdateType {
    BuySide = 0x38,
    SellSide = 0x35,
}
