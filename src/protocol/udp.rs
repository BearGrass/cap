use super::parser::{UdpHeader, PacketParser};

#[derive(Debug)]
pub struct UdpParser;

impl UdpParser {
    pub fn new() -> Self {
        Self
    }
}

impl PacketParser for UdpParser {
    type Output = UdpHeader;
    type Error = UdpError;

    fn parse(&self, data: &[u8]) -> Result<Self::Output, Self::Error> {
        if data.len() < 8 {
            return Err(UdpError::TooShort);
        }

        let src_port = u16::from_be_bytes([data[0], data[1]]);
        let dst_port = u16::from_be_bytes([data[2], data[3]]);
        let length = u16::from_be_bytes([data[4], data[5]]);

        Ok(UdpHeader {
            src_port,
            dst_port,
            length,
        })
    }

    fn protocol_name(&self) -> &'static str {
        "UDP"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UdpError {
    #[error("Packet too short for UDP header (minimum 8 bytes)")]
    TooShort,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udp_parser() {
        let parser = UdpParser::new();
        let mut data = [0u8; 8];
        data[0] = 0x00; data[1] = 0x35; // src_port: 53 (DNS)
        data[2] = 0x1f; data[3] = 0x90; // dst_port: 8080
        data[4] = 0x00; data[5] = 0x35; // length: 53

        let result = parser.parse(&data).unwrap();
        assert_eq!(result.src_port, 53);
        assert_eq!(result.dst_port, 8080);
        assert_eq!(result.length, 53);
    }
}
