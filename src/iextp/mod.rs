#![allow(unused_variables, dead_code)]
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::io::Cursor;

pub mod tops;

const SEGMENT_HEADER_SIZE: u16 = 40;
const UNIX_YEAR: i64 = 1_000_000_000;

#[derive(Deserialize)]
pub struct HIST {
    link: String,
    date: String,
    feed: String,
    version: String,
    protocol: String,
    size: i64,
}

pub trait Unmarshal {
    fn unmarshal(buf: &[u8]) -> Self
    where
        Self: Sized;

    // REVIEW HACK TEMP(parse_string): Cannot find a "normal" & efficient way to
    // deal with strings, setting signature return var as Vec<char> for now.
    fn parse_string(buf: &[u8]) -> Vec<char> {
        buf.iter()
            .map(|byte| *byte as char)
            .filter(|c| *c != ' ')
            .collect()
    }

    //len 8
    fn parse_price(buf: &[u8]) -> f64 {
        let n = Cursor::new(buf).read_i64::<LittleEndian>().unwrap();
        (n as f64) / 10000.00
    }

    fn parse_u32(buf: &[u8]) -> u32 {
        Cursor::new(&buf).read_u32::<LittleEndian>().unwrap()
    }

    fn parse_u64(buf: &[u8]) -> u64 {
        Cursor::new(&buf).read_u64::<LittleEndian>().unwrap()
    }

    // len 8
    fn parse_timestamp(buf: &[u8]) -> DateTime<Utc> {
        let timestamp = Cursor::new(buf).read_u64::<LittleEndian>().unwrap() as i64;

        let secs: i64 = timestamp / UNIX_YEAR;
        let n_secs = (timestamp % UNIX_YEAR) as u32;

        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(secs, n_secs), Utc)
    }

    // len 4
    fn parse_event_time(buf: &[u8]) -> DateTime<Utc> {
        let timestamp = Cursor::new(buf).read_u32::<LittleEndian>().unwrap() as i64;
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;

    struct Mock;
    impl Unmarshal for Mock {
        fn unmarshal(buf: &[u8]) -> Self {
            Mock
        }
    }

    #[test]
    fn parse_string() {
        // ZIEXT
        let symbol_literal = &[0x5a, 0x49, 0x45, 0x58, 0x54, 0x20, 0x20, 0x20];
        // HACK -- may want to change functionality of parse_string method.
        let parsed: String = Mock::parse_string(symbol_literal).iter().collect();
        assert_eq!(String::from("ZIEXT"), parsed);
    }

    #[test]
    fn parse_price() {
        // 99.05
        let price_literal = &[0x24, 0x1d, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(99.05, Mock::parse_price(price_literal));
    }

    #[test]
    fn parse_timestamp_secs() {
        // 2017-04-17 17:00:00
        let time_literal = &[0x00, 0xa0, 0x99, 0x97, 0xe9, 0x3d, 0xb6, 0x14];
        let date_expected = Utc.ymd(2017, 04, 17).and_hms(17, 0, 0);
        assert_eq!(date_expected, Mock::parse_timestamp(time_literal));
    }

    #[test]
    fn parse_timestamp_nsecs() {
        // 2016-08-23 15:30:32.572715948;
        let time_literal = &[0xac, 0x63, 0xc0, 0x20, 0x96, 0x86, 0x6d, 0x14];
        let date_expected = Utc.ymd(2016, 08, 23).and_hms_nano(19, 30, 32, 572_715_948);
        assert_eq!(date_expected, Mock::parse_timestamp(time_literal));
    }

    #[test]
    fn parse_event_time() {
        // 2017-04-17 16:00:00
        let time_literal = &[0x80, 0xe6, 0xf4, 0x58];
        let expected = Utc.ymd(2017, 4, 17).and_hms(16, 0, 0);
        assert_eq!(expected, Mock::parse_event_time(time_literal));
    }

}

// #[derive(Debug, Default)]
// struct Segment {
//     Header: SegmentHeader,
//     Messages: Vec<Message>,
// }

struct SegmentHeader {
    // Version of the IEX-TP protocol.
    version: u8,
    // Reserved byte.
    // REVIEW: ^...?

    // A unique identifier for the higher-layer specification that describes
    // the messages contaiend within a segment. See the higher-layer protocol
    // specification for the protocol's message identification in IEX-TP.
    message_protocol_id: u16,

    // An identifier for a given stream of bytes/sequenced messages. Messages
    // received from multiple sources which use the same Channel ID are
    // guaranteed to be duplicates by sequence number and/or offset. See the
    // higher-layer protocol specification for the protocol's channel
    // identification on IEX-TP.
    channel_id: u32,

    // SessionID uniquely identifies a stream of messages produced by the
    // system. A given message is uniquely identified within a message
    // protocol by its Session ID and Sequence Number.
    session_id: u32,

    // PayloadLength is an unsigned binary count representing the number
    // of bytes contained in the segment's payload. Note that the Payload
    // Length field value does not include the length of the IEX-TP
    // header.
    payload_length: u16,

    // MessageCount is a count representing the number of Message Blocks
    // in the segment.
    message_count: u16,

    // StreamOffset is a counter representing the byte offset of the payload
    // in the data stream.
    stream_offset: i64,

    // FirstMessageSequenceNumber is a counter representing the sequence
    // number of the first message in the segment. If there is more than one
    // message in a segment, all subsequent messages are implicitly
    // numbered sequentially.
    first_message_sequence_number: i64,
    // The time the outbound segment was sent as set by the sender.
    send_time: DateTime<Utc>,
}
