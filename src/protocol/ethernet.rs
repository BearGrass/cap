use super::parser::{EthernetHeader, PacketParser};

#[derive(Debug)]
pub struct EthernetParser;

impl EthernetParser {
    pub fn new() -> Self {
        Self
    }
}

impl PacketParser for EthernetParser {
    type Output = EthernetHeader;
    type Error = EthernetError;

    fn parse(&self, data: &[u8]) -> Result<Self::Output, Self::Error> {
        if data.len() < 14 {
            return Err(EthernetError::TooShort);
        }

        let dst_mac = format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            data[0], data[1], data[2], data[3], data[4], data[5]
        );
        let src_mac = format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            data[6], data[7], data[8], data[9], data[10], data[11]
        );
        let ether_type = u16::from_be_bytes([data[12], data[13]]);

        Ok(EthernetHeader {
            src_mac,
            dst_mac,
            ether_type,
        })
    }

    fn protocol_name(&self) -> &'static str {
        "Ethernet"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EthernetError {
    #[error("Packet too short for Ethernet header (minimum 14 bytes)")]
    TooShort,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_parser() {
        let parser = EthernetParser::new();
        // Sample Ethernet frame: dst_mac(6) + src_mac(6) + ether_type(2)
        let data = [
            0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e, // dst_mac
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // src_mac
            0x08, 0x00, // IPv4
        ];

        let result = parser.parse(&data).unwrap();
        assert_eq!(result.dst_mac, "00:1a:2b:3c:4d:5e");
        assert_eq!(result.src_mac, "00:11:22:33:44:55");
        assert_eq!(result.ether_type, 0x0800);
    }

    #[test]
    fn test_ethernet_parser_too_short() {
        let parser = EthernetParser::new();
        let data = [0x00, 0x1a, 0x2b]; // Too short

        assert!(parser.parse(&data).is_err());
    }
}
