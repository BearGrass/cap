use super::parser::{IpHeader, PacketParser};

#[derive(Debug)]
pub struct IpParser;

impl IpParser {
    pub fn new() -> Self {
        Self
    }
}

impl PacketParser for IpParser {
    type Output = IpHeader;
    type Error = IpError;

    fn parse(&self, data: &[u8]) -> Result<Self::Output, Self::Error> {
        if data.len() < 20 {
            return Err(IpError::TooShort);
        }

        let version = (data[0] >> 4) & 0x0F;

        match version {
            4 => self.parse_ipv4(data),
            6 => self.parse_ipv6(data),
            _ => Err(IpError::UnknownVersion(version)),
        }
    }

    fn protocol_name(&self) -> &'static str {
        "IP"
    }
}

impl IpParser {
    fn parse_ipv4(&self, data: &[u8]) -> Result<IpHeader, IpError> {
        if data.len() < 20 {
            return Err(IpError::TooShort);
        }

        let version = (data[0] >> 4) & 0x0F;
        let _ihl = (data[0] & 0x0F) * 4;
        let ttl = data[8];
        let protocol = data[9];

        let src = format!("{}.{}.{}.{}", data[12], data[13], data[14], data[15]);
        let dst = format!("{}.{}.{}.{}", data[16], data[17], data[18], data[19]);

        Ok(IpHeader {
            version,
            src,
            dst,
            protocol,
            ttl,
        })
    }

    fn parse_ipv6(&self, data: &[u8]) -> Result<IpHeader, IpError> {
        if data.len() < 40 {
            return Err(IpError::TooShort);
        }

        let version = (data[0] >> 4) & 0x0F;
        let next_header = data[6];
        let hop_limit = data[7];

        let src = format!(
            "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
            data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23]
        );
        let dst = format!(
            "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
            data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31],
            data[32], data[33], data[34], data[35], data[36], data[37], data[38], data[39]
        );

        Ok(IpHeader {
            version,
            src,
            dst,
            protocol: next_header,
            ttl: hop_limit,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IpError {
    #[error("Packet too short for IP header")]
    TooShort,
    #[error("Unknown IP version: {0}")]
    UnknownVersion(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_parser() {
        let parser = IpParser::new();
        // Minimal IPv4 header (20 bytes)
        let mut data = [0u8; 20];
        data[0] = 0x45; // Version 4, IHL 5
        data[8] = 64;   // TTL
        data[9] = 6;    // Protocol (TCP)
        data[12] = 192; data[13] = 168; data[14] = 1; data[15] = 100; // src: 192.168.1.100
        data[16] = 8; data[17] = 8; data[18] = 8; data[19] = 8;       // dst: 8.8.8.8

        let result = parser.parse(&data).unwrap();
        assert_eq!(result.version, 4);
        assert_eq!(result.src, "192.168.1.100");
        assert_eq!(result.dst, "8.8.8.8");
        assert_eq!(result.ttl, 64);
    }
}
