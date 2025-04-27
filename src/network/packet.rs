use std::io::Error;

use super::protocol::RequestType;

pub struct Packet {
    pub header: Header,
    pub payload: Box<[u8]>,
}

pub struct Header {
    pub htype: RequestType,
    pub checksum: i16,
    pub length: i16,
}

impl Packet {
    pub fn parse_packet(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 6 {
            return Err(Error::new(std::io::ErrorKind::Other, ""));
        }

        let header_bytes = &bytes[..6];
        let payload_bytes = &bytes[6..];

        let header = Header::parse_header(header_bytes)?;

        return Ok(Self {
            header: header,
            payload: payload_bytes.to_owned().into_boxed_slice(),
        });
    }
}

impl Header {
    pub fn parse_header(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 5 {
            return Err(Error::new(std::io::ErrorKind::Other, ""));
        }

        match RequestType::try_from(bytes[0]) {
            Err(_) => {
                return Err(Error::new(std::io::ErrorKind::Other, ""));
            }
            Ok(request_type) => {
                let checksum: i16 = u16::from_be_bytes([bytes[3], bytes[4]]) as i16;
                let payload_length: i16 = u16::from_be_bytes([bytes[1], bytes[2]]) as i16;
                return Ok(Self {
                    htype: request_type,
                    length: payload_length,
                    checksum,
                });
            }
        }
    }
}
