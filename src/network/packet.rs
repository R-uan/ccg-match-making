use super::protocol::HeaderType;
use crate::checksum::Checksum;
use crate::utils::errors::ProtocolError;
use std::io::Error;

pub struct Packet {
    pub header: Header,
    pub payload: Box<[u8]>,
}

pub struct Header {
    pub header_type: HeaderType,
    pub checksum: i16,
    pub length: i16,
}

impl Header {
    pub fn new(header_type: HeaderType, payload: &[u8]) -> Self {
        Self {
            header_type: header_type,
            length: payload.len() as i16,
            checksum: Checksum::new(&payload) as i16,
        }
    }

    pub fn wrap_header(&self) -> Box<[u8]> {
        let checksum: i16 = self.checksum;
        let payload_length: i16 = self.length;
        let header_type: u8 = self.header_type.to_owned() as u8;

        Box::new([
            header_type,
            ((payload_length >> 8) & 0xFF) as u8,
            (payload_length & 0xFF) as u8,
            ((checksum >> 8) & 0xFF) as u8,
            (checksum & 0xFF) as u8,
            0x0A,
        ])
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() != 6 || bytes[5] != 0x0A {
            return Err(ProtocolError::InvalidHeader(
                "number of bytes invalid.".to_string(),
            ));
        }

        match HeaderType::try_from(bytes[0]) {
            Err(_) => Err(ProtocolError::InvalidHeader(
                "invalid header type.".to_string(),
            )),

            Ok(header_type) => {
                let checksum: i16 = u16::from_be_bytes([bytes[3], bytes[4]]) as i16;
                let payload_length: i16 = u16::from_be_bytes([bytes[1], bytes[2]]) as i16;

                Ok(Self {
                    header_type,
                    checksum,
                    length: payload_length,
                })
            }
        }
    }
}

impl Packet {
    pub fn new(header_type: HeaderType, payload: Vec<u8>) -> Self {
        let header = Header::new(header_type, &payload);
        let payload = payload.into_boxed_slice();
        Self { header, payload }
    }

    pub fn parse_packet(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() < 6 {
            return Err(ProtocolError::InvalidHeader(
                "number of bytes invalid.".to_string(),
            ));
        }

        let header_bytes = &bytes[..6];
        let payload_bytes = &bytes[6..];

        let header = Header::from_bytes(header_bytes)?;

        Ok(Self {
            header,
            payload: payload_bytes.to_owned().into_boxed_slice(),
        })
    }

    pub fn wrap_packet(&self) -> Box<[u8]> {
        let header = self.header.wrap_header();
        let mut packet = Vec::with_capacity(header.len() + self.payload.len());

        packet.extend_from_slice(&header);
        packet.extend_from_slice(&self.payload);

        packet.into_boxed_slice()
    }
}
